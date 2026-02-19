#ifndef GRAPHICS_EVENT_H
#define GRAPHICS_EVENT_H

enum graphics_event_t
{
    EVENT_FOCUS,
    EVENT_UNFOCUS,

    EVENT_SCROLL_LEFT,
    EVENT_SCROLL_RIGHT,

    EVENT_CLICK,
    EVENT_BACK
};

enum user_event_t
{
    EVENT_ONCLICK
};

#endif //GRAPHICS_EVENT_H