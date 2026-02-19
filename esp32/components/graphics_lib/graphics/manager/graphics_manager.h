#ifndef GRAPHICS_GRAPHICS_MANAGER_H
#define GRAPHICS_GRAPHICS_MANAGER_H

#include "freertos/FreeRTOS.h"
#include "freertos/queue.h"
#include <memory>
#include <vector>
#include <string>
#include <functional>

#include <graphics/screen/screen.h>
#include <graphics/drivers/driver.h>

class GraphicsManager;
typedef std::function<screen_t(GraphicsManager&)> screen_factory_t;

class GraphicsManager
{
    std::vector<graphics_driver_t> drivers;

    std::unordered_map<std::string, screen_factory_t> screen_factories;

    std::vector<std::string> backstack{};
    std::unique_ptr<Screen> current_screen = nullptr;

    void navigate_screen(screen_t);

    const char* TAG = "GraphicsManager";
public:
    void load_screen(const std::string&, const screen_factory_t&);
    void install_driver(graphics_driver_t);

    void navigate(const std::string&);

    void update() const;

    void send_event(graphics_event_t);
};

#endif //GRAPHICS_GRAPHICS_MANAGER_H