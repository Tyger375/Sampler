#ifndef GRAPHICS_LOGGER_H
#define GRAPHICS_LOGGER_H

#include "../driver.h"

class logger_driver : public GraphicsDriver
{
public:
    void init() override;
    void draw(std::vector<std::string>) override;
    void clear() override;
};


#endif //GRAPHICS_LOGGER_H