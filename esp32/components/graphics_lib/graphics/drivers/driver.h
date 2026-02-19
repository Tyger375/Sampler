#ifndef GRAPHICS_DRIVER_H
#define GRAPHICS_DRIVER_H

#include <vector>
#include <memory>
#include <string>

class GraphicsDriver
{
public:
    virtual ~GraphicsDriver() = default;

    virtual void init() = 0;
    virtual void draw(std::vector<std::string>) = 0;
    virtual void clear() = 0;
};

typedef std::unique_ptr<GraphicsDriver> graphics_driver_t;

#endif //GRAPHICS_DRIVER_H