<script lang="ts">
    import {
        Select,
        SelectTrigger,
        SelectContent,
        SelectGroup,
        SelectLabel,
        SelectItem
    } from "@/components/ui/select";

    const noteLetters = [
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"
    ];

    const midiNotes = Array.from({ length: 128 }, (_, midiNumber) => {
        const octave = Math.floor((midiNumber - 21) / 12) + 1;
        const noteLetter = noteLetters[midiNumber % 12];

        return {
            number: midiNumber,
            label: `${noteLetter}${octave}`
        };
    }).filter((item) => item.number >= 21);

    let {
        value = $bindable(),
        onValueChange
    }: {
        value: number | undefined,
        onValueChange?: (value: number) => void
    } = $props();

    let selectedLabel = $derived(
        midiNotes.find(n => n.number === value)?.label ?? "Select Note..."
    );
</script>

<Select type="single"
        value={value?.toString()}
        onValueChange={(newValue) => {
            const numValue = parseInt(newValue, 10);
            value = numValue;
            onValueChange?.(numValue);
        }}
>
    <SelectTrigger class="w-45 bg-zinc-900 border-zinc-800 text-white">
        {selectedLabel}
    </SelectTrigger>
    <SelectContent class="bg-zinc-900 border-zinc-800 text-white max-h-75 overflow-y-auto">
        <SelectGroup>
            <SelectLabel class="text-zinc-400">Note</SelectLabel>
            {#each midiNotes as note}
                <SelectItem value={note.number.toString()}>
                    {note.label}
                </SelectItem>
            {/each}
        </SelectGroup>
    </SelectContent>
</Select>