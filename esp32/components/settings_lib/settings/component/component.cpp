#include "component.h"

void SettingsComponent::init(const QueueHandle_t handle)
{
    save_queue = handle;

    on_load();
}

void SettingsComponent::commit()
{
    const auto item = this;
    xQueueSend(save_queue, &item, 0);
}

std::string SettingsComponent::direct_read(const std::string& arg)
{
    return {};
}

bool SettingsComponent::direct_write(const std::string& buffer, const std::string& arg)
{
    return false;
}
