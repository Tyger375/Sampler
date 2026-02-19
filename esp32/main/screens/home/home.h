#ifndef SAMPLER_HOME_SCREEN_H
#define SAMPLER_HOME_SCREEN_H

#include <graphics/manager/graphics_manager.h>
#include <graphics/screen/screen.h>

class HomeScreen : public Screen
{
public:
    explicit HomeScreen(GraphicsManager& graphics_manager);

    //bool on_custom_event(uint32_t) override;
};

#endif //SAMPLER_HOME_SCREEN_H