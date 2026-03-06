#ifndef SAMPLER_GROUP_H
#define SAMPLER_GROUP_H

#include <vector>
#include <memory>
#include <utility>
#include "settings/component/component.h"

class SettingsGroup
{
    std::vector<component_t> components;
public:
    const std::string prefix;

    SettingsGroup(std::string prefix) : prefix(std::move(prefix)) {}
    virtual ~SettingsGroup() = default;

    virtual void init();
};

typedef std::unique_ptr<SettingsGroup> group_t;

#endif //SAMPLER_GROUP_H