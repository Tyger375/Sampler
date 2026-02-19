#ifndef SAMPLER_EVENT_H
#define SAMPLER_EVENT_H

enum graphics_event_t
{
    EVENT_ON_NAVIGATE,

    EVENT_SELECT,
    EVENT_UNSELECT,

    EVENT_FOCUS,
    // EVENT_UNFOCUS,

    EVENT_SCROLL_LEFT,
    EVENT_SCROLL_RIGHT,
    EVENT_OK,
    EVENT_BACK
};

enum user_event_t
{
    EVENT_ONCLICK
};

#endif //SAMPLER_EVENT_H