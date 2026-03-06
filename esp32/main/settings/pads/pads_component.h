#ifndef SAMPLER_PADS_COMPONENT_H
#define SAMPLER_PADS_COMPONENT_H

#include <settings/component/component.h>

class PadsComponent : public SettingsComponent
{
public:
    PadsComponent();

    void on_load() override;
    void save() override;
};


#endif //SAMPLER_PADS_COMPONENT_H