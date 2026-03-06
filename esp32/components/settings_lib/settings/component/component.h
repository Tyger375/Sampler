#ifndef SAMPLER_COMPONENT_H
#define SAMPLER_COMPONENT_H

#include <freertos/FreeRTOS.h>
#include <string>
#include <memory>
#include <utility>

class SettingsComponent
{
    QueueHandle_t save_queue = nullptr;

public:
    const std::string id;
    void init(QueueHandle_t);

    virtual void on_load() = 0;

    virtual void save() = 0;
    virtual void commit();

    SettingsComponent(std::string id) : id(std::move(id)) {}
    virtual ~SettingsComponent() = default;
};

typedef std::unique_ptr<SettingsComponent> component_t;

#endif //SAMPLER_COMPONENT_H