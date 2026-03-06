#include "component.h"

void SettingsComponent::init(const QueueHandle_t handle)
{
    save_queue = handle;

    on_load();
}

void SettingsComponent::commit()
{
    SettingsComponent* item = this;
    xQueueSend(save_queue, &item, 0);
}
