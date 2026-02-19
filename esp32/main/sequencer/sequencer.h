#ifndef SAMPLER_SEQUENCER_H
#define SAMPLER_SEQUENCER_H

#include <atomic>
#include <vector>

typedef enum : uint8_t
{
    SEQ_RES_QUARTER = 1,
    SEQ_RES_HALF_BEAT = 2,
    SEQ_RES_BEAT = 4,
    SEQ_RES_LOOP = 16
} sequencer_resolution_t;

typedef struct
{
    uint8_t loops;
    sequencer_resolution_t resolution;
    uint8_t note;
    bool trigger;
    std::vector<uint8_t> triggers;
} sequencer_track_t;

class Sequencer
{
public:
    std::vector<sequencer_track_t> tracks;
    std::atomic<bool> enable;

    static Sequencer& instance()
    {
        static Sequencer instance;
        return instance;
    }

    Sequencer(const Sequencer&) = delete;
    void operator=(const Sequencer&) = delete;

    void step_trigger_on(uint8_t);
    void step_trigger_off(uint8_t);

    void set_loops_num(const uint8_t loops)
    {
        loops_num = loops;
    }
private:
    uint8_t current_loop = 0;
    uint8_t loops_num = 1;

    void handle_track_on(sequencer_track_t&, uint8_t);
    void handle_track_off(sequencer_track_t&, uint8_t);

    Sequencer();
};


#endif //SAMPLER_SEQUENCER_H