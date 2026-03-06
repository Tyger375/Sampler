#include "graphics_manager.h"

#include <esp_log.h>

void GraphicsManager::navigate_screen(screen_t screen)
{
    if (current_screen != nullptr)
        current_screen->on_end();
    current_screen = std::move(screen);

    current_screen->on_start();
}

void GraphicsManager::load_screen(const std::string& id, const screen_factory_t& factory)
{
    if (screen_factories.contains(id))
    {
        ESP_LOGE(TAG, "Factory for screen %s already exists", id.c_str());
        return;
    }
    screen_factories[id] = factory;
}

void GraphicsManager::install_driver(graphics_driver_t driver)
{
    driver->init();
    drivers.push_back(std::move(driver));
}

void GraphicsManager::navigate(const std::string& id)
{
    if (!screen_factories.contains(id))
    {
        ESP_LOGE(TAG, "Factory for screen %s does not exist", id.c_str());
        return;
    }

    if (current_screen != nullptr)
    {
        backstack.push_back(current_screen->id);
    }

    navigate_screen(screen_factories[id](*this));
}

void GraphicsManager::navigate_back()
{
    if (backstack.empty())
    {
        ESP_LOGE(TAG, "Backstack is empty");
        return;
    }

    auto id = backstack.back();
    backstack.pop_back();

    if (!screen_factories.contains(id))
    {
        ESP_LOGE(TAG, "Factory for screen %s does not exist", id.c_str());
        return;
    }

    navigate_screen(screen_factories[id](*this));
}

void GraphicsManager::update() const
{
    if (current_screen == nullptr)
    {
        ESP_LOGE(TAG, "Current screen is null");
        return;
    }

    const auto rows = current_screen->render(2);

    for (auto& driver : drivers)
    {
        driver->clear();
        driver->draw(rows);
    }
}

void GraphicsManager::send_event(graphics_event_t event)
{
    if (current_screen == nullptr)
    {
        ESP_LOGE(TAG, "Current screen is null");
        return;
    }
    switch (event)
    {
    case EVENT_SCROLL_RIGHT:
        current_screen->on_scroll(true);
        break;
    case EVENT_SCROLL_LEFT:
        current_screen->on_scroll(false);
        break;
    case EVENT_CLICK:
        current_screen->on_click();
        break;
    case EVENT_BACK:
        if (!current_screen->on_back())
            navigate_back();
        break;
    default:
        {
            ESP_LOGE(TAG, "Unknown event type");
        } break;
    }
}

bool GraphicsManager::send_custom_event(const uint32_t event) const
{
    if (current_screen == nullptr)
    {
        ESP_LOGE(TAG, "Current screen is null");
        return false;
    }

    return current_screen->on_custom_event(event);
}
