<script lang="ts">
    import {listen} from "@tauri-apps/api/event";
    import {onMount} from "svelte";
    import AudioWaveform from "svelte-audio-waveform";

    import {
        Select,
        SelectTrigger,
        SelectContent,
        SelectGroup,
        SelectLabel,
        SelectItem
    } from "@/components/ui/select";
    import { Button } from "$lib/components/ui/button";
    import {
        Dialog,
        DialogContent,
        DialogHeader,
        DialogTitle,
        DialogTrigger
    } from "$lib/components/ui/dialog";
    import { Pad } from "$lib/components/pad";
    import {NoteSelect} from "@/components/ui/note_select";
    import {invoke} from "@tauri-apps/api/core";

    let addAudioTrackDialog = $state(false);
    let showPadConfigDialog = $state(false);
    let padConfigDialog: number = $state(0);

    interface PadConfig {
        id: number,
        note: number,
        track_id: number
    }

    interface AudioTrack {
        label: string,
        id: number
    }

    interface AudioFile {
        path: string,
        data: number[],
        channels: number,
        sample_rate: number
    }

    let pad_configs: PadConfig[] = $state([])
    let pads_states: boolean[] = $state([]);

    $effect(() => {
        console.log("setting pads");
        invoke("set_pad_configs", { padConfigs: pad_configs });
    });

    let tracks: AudioTrack[] = $state([]);

    onMount(() => {
        const pads = document.getElementsByClassName("pad");
        Array.from(pads).forEach((item) => {
            item.classList.remove("pressed");
        });

        const pad_configs_promise = invoke<PadConfig[]>("get_pad_configs");
        pad_configs_promise.then((res) => {
            pad_configs = res;
            pads_states = Array(pad_configs.length).fill(false);
        });

        invoke<AudioTrack[]>("get_tracks").then((res) => {
            tracks = res;
        });

        const unlistenPromise = listen<[number, boolean]>("pad-event", (event) => {
            const [pad_id, pressed] = event.payload;

            const index = pad_id - 1;

            if (index >= 0 && index < pads_states.length) {
                pads_states[index] = pressed;
            }
        });

        return () => {
            unlistenPromise.then((unlisten) => unlisten());
        }
    });

    let audio_file: AudioFile | null = $state(null);
</script>

<main class="bg-background p-10">
    <div class="flex flex-col gap-5">
        <div class="w-full flex justify-center items-center">
            <div class="w-full max-w-md grid grid-flow-col grid-rows-2 gap-1">
                {#each pads_states as isPressed, i}
                    <Pad pad_id={i + 1}
                         pressed={isPressed}
                         onclick={() => {
                             padConfigDialog = i;
                             showPadConfigDialog = true;
                         }}
                    ></Pad>
                {/each}
            </div>
        </div>
        <div>
            <h1 class="text-4xl">Audio Tracks</h1>

            <Button variant="outline" onclick={() => addAudioTrackDialog = true}>
                Add Track
            </Button>

            {#each tracks as track}
                <button onclick={async () => {
                    const res = await invoke<AudioFile | null>("get_audio_file", { trackId: track.id });
                    console.log(res);
                    audio_file = res;
                }}>
                    {track.label} ({track.id})
                </button>
            {/each}
        </div>
        {#if (audio_file)}
            <div>
                <AudioWaveform
                        peaks={audio_file.data}
                        position={0}
                        gradientColors={[]}
                        progressGradientColors={[]}
                        height={100}
                        width={500}
                        barWidth={1}
                    />
            </div>
        {/if}
    </div>

    <Dialog bind:open={addAudioTrackDialog}>
        <DialogTrigger>

        </DialogTrigger>
        <DialogContent class="sm:max-w-106.25 bg-[#1a1a1a] text-white border-[#333]">
            <DialogHeader>
                <DialogTitle>Add Audio Track</DialogTitle>
            </DialogHeader>
            Test!
        </DialogContent>
    </Dialog>

    <Dialog bind:open={showPadConfigDialog}>
        <DialogTrigger>

        </DialogTrigger>
        <DialogContent>
            <DialogHeader>
                <DialogTitle>Configure Pad {padConfigDialog + 1}</DialogTitle>
            </DialogHeader>

            {@const currentPad = pad_configs[padConfigDialog]}
            {@const track = tracks.find((t) => t.id == currentPad.track_id)}

            <Select type="single"
                    value={track?.label ?? ""}
                    onValueChange={(newValue) => {
                        currentPad.track_id = parseInt(newValue, 10);
                    }}>
                <SelectTrigger class="w-45 bg-zinc-900 border-zinc-800 text-white">
                    {track?.label}
                </SelectTrigger>
                <SelectContent class="bg-zinc-900 border-zinc-800 text-white max-h-75 overflow-y-auto">
                    <SelectGroup>
                        <SelectLabel class="text-zinc-400">Track</SelectLabel>

                        {#each tracks as track}
                            <SelectItem value={track.id.toString()}>
                                {track.label}
                            </SelectItem>
                        {/each}
                    </SelectGroup>
                </SelectContent>
            </Select>

            <NoteSelect value={currentPad.note}
                        onValueChange={(value) => {
                            currentPad.note = value;
                        }}/>
        </DialogContent>
    </Dialog>
</main>