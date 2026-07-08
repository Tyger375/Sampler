<script lang="ts">
    type PadProps = {
        pad_id: number,
        pressed: boolean,
        onclick: () => void
    };
    let { pad_id, pressed, onclick }: PadProps = $props();
</script>

<button class="{pressed ? 'bg-primary' : 'bg-secondary'} w-full aspect-square flex justify-center items-center text-lg rounded-lg select-none cursor-pointer"
     id="pad-{pad_id}"
     onclick={onclick}
>{pad_id}</button>