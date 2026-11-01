#!/bin/sh
SD="${OMEGA_ROOT:-/mnt/sdcard}"
SYS="$SD/System"
RUN="${AGS_RUN:-/run}"
export OMEGA_ROOT="$SD"

log() {
	echo "$(date '+%H:%M:%S') $*" >> "$SD/omega-zero-session.log" 2>/dev/null
}

pid="$(cat "$RUN/omega-led.pid" 2>/dev/null)"
if [ -z "$pid" ] || [ ! -d "/proc/$pid" ]; then
	sh "$SYS/led.sh" once
	sh "$SYS/led.sh" loop > /dev/null 2>&1 &
	echo $! > "$RUN/omega-led.pid"
fi

echo "|/bin/sh $SYS/coresave.sh %p %s %e %t" > /proc/sys/kernel/core_pattern 2>/dev/null

if [ -f "$SD/RUN_PROBE" ]; then
	rm -f "$SD/RUN_PROBE"
	log "probe $(AGS_PROBE_NET_WAIT_S=15 sh "$SYS/probe.sh" 2>&1)"
fi

returning_from_standalone=0
while :; do
	[ -f "$SD/omega-zero.log" ] && mv -f "$SD/omega-zero.log" "$SD/omega-zero.log.1"
	log "exec $SYS/omega-zero"
	OMEGA_RETURNING_FROM_STANDALONE="$returning_from_standalone" \
		/lib/ld-linux-aarch64.so.1 "$SYS/omega-zero" > "$SD/omega-zero.log" 2>&1
	status=$?
	returning_from_standalone=0
	[ "$status" -eq 42 ] || exit "$status"
	if [ -f "$SYS/hypseus.launch" ]; then
		sh "$SYS/hypseus/start.sh" "$SYS/hypseus.launch" >> "$SD/hypseus.log" 2>&1
		standalone_status=$?
		[ "$standalone_status" -eq 0 ] || log "Hypseus failed ($standalone_status)"
		rm -f "$SYS/hypseus.launch"
	else
		sh "$SYS/bigpemu/start.sh" "$SYS/bigpemu.launch" >> "$SD/bigpemu.log" 2>&1
		standalone_status=$?
		[ "$standalone_status" -eq 0 ] || log "BigPEmu failed ($standalone_status)"
		rm -f "$SYS/bigpemu.launch"
	fi
	# EGL and the framebuffer must be released while BigPEmu owns them, so omega-zero starts a
	# new process here. Mark this particular start as a return: it draws the shelf immediately,
	# without replaying the boot logo or opening as though the console had rebooted.
	returning_from_standalone=1
done
