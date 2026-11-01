#include <SDL.h>
#include <stdio.h>

int main(void) {
    SDL_SetHint(SDL_HINT_JOYSTICK_ALLOW_BACKGROUND_EVENTS, "1");
    if (SDL_Init(SDL_INIT_JOYSTICK | SDL_INIT_GAMECONTROLLER) != 0) {
        fprintf(stderr, "BigPEmu SDL probe: init failed: %s\n", SDL_GetError());
        return 1;
    }

    const int count = SDL_NumJoysticks();
    fprintf(stderr, "BigPEmu SDL probe: joysticks=%d\n", count);
    for (int index = 0; index < count; ++index) {
        char guid[33];
        SDL_JoystickGetGUIDString(SDL_JoystickGetDeviceGUID(index), guid, sizeof(guid));
        SDL_Joystick *stick = SDL_JoystickOpen(index);
        fprintf(stderr,
                "BigPEmu SDL probe: index=%d name=%s guid=%s controller=%d axes=%d buttons=%d hats=%d open=%s\n",
                index,
                SDL_JoystickNameForIndex(index) ? SDL_JoystickNameForIndex(index) : "?",
                guid,
                SDL_IsGameController(index),
                stick ? SDL_JoystickNumAxes(stick) : -1,
                stick ? SDL_JoystickNumButtons(stick) : -1,
                stick ? SDL_JoystickNumHats(stick) : -1,
                stick ? "yes" : SDL_GetError());
        if (stick) {
            SDL_JoystickClose(stick);
        }
    }
    SDL_Quit();
    return 0;
}
