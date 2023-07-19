#include <windows.h>

int CALLBACK EnumWindowsProc(HWND hwnd, LPARAM lParam) {
    HWND p = FindWindowEx(hwnd, NULL, "SHELLDLL_DefView", NULL);
    HWND* ret = (HWND*)lParam;

    if (p) {
        *ret = FindWindowEx(NULL, hwnd, "WorkerW", NULL);
    }
    return 1;
}

void* get_wallpaper_hwnd(void) {
    HWND progman = FindWindow("ProgMan", NULL);
    SendMessageTimeout(progman, 0x052C, 0, 0, SMTO_NORMAL, 1000, NULL);
    HWND wallpaper_hwnd = NULL;
    EnumWindows(EnumWindowsProc, (LPARAM)&wallpaper_hwnd);
    return (void*)wallpaper_hwnd;
}
