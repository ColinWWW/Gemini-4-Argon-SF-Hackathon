// Emacs style mode select   -*- C++ -*-
//-----------------------------------------------------------------------------
//
// DESCRIPTION:
//	Headless video driver for Referee (pixel-verified frame dumper).
//
//-----------------------------------------------------------------------------

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>

#ifdef _WIN32
#include <direct.h>
#include <io.h>
#endif

#include "doomdef.h"
#include "doomstat.h"
#include "i_system.h"
#include "v_video.h"
#include "i_video.h"

static byte current_palette[768];
static int frame_count = 0;

static void ensure_dir(const char* dir)
{
    char tmp[512];
    char* p = NULL;
    size_t len;

    snprintf(tmp, sizeof(tmp), "%s", dir);
    len = strlen(tmp);
    if (len > 0 && (tmp[len - 1] == '/' || tmp[len - 1] == '\\'))
        tmp[len - 1] = 0;

    for (p = tmp + 1; *p; p++)
    {
        if (*p == '/' || *p == '\\')
        {
            *p = 0;
#ifdef _WIN32
            _mkdir(tmp);
#else
            mkdir(tmp, 0777);
#endif
            *p = '/';
        }
    }
#ifdef _WIN32
    _mkdir(tmp);
#else
    mkdir(tmp, 0777);
#endif
}

void I_InitGraphics(void)
{
    if (!screens[0])
    {
        screens[0] = (byte*)malloc(SCREENWIDTH * SCREENHEIGHT);
        memset(screens[0], 0, SCREENWIDTH * SCREENHEIGHT);
    }
}

void I_ShutdownGraphics(void)
{
}

void I_SetPalette(byte* palette)
{
    if (palette)
        memcpy(current_palette, palette, 768);
}

void I_UpdateNoBlit(void)
{
}

void I_FinishUpdate(void)
{
    char filename[512];
    const char* frame_dir = getenv("DOOM_FRAME_DIR");
    FILE* f;

    if (!frame_dir || !*frame_dir)
        frame_dir = "frames/c";

    ensure_dir(frame_dir);

    snprintf(filename, sizeof(filename), "%s/%05d.raw", frame_dir, frame_count++);
    f = fopen(filename, "wb");
    if (f)
    {
        if (screens[0])
            fwrite(screens[0], 1, SCREENWIDTH * SCREENHEIGHT, f);
        else
        {
            static byte empty_screen[SCREENWIDTH * SCREENHEIGHT];
            fwrite(empty_screen, 1, SCREENWIDTH * SCREENHEIGHT, f);
        }
        fwrite(current_palette, 1, 768, f);
        fclose(f);
    }
}

void I_ReadScreen(byte* scr)
{
    if (screens[0] && scr)
        memcpy(scr, screens[0], SCREENWIDTH * SCREENHEIGHT);
}


void I_StartTic(void)
{
}

void I_StartFrame(void)
{
}
