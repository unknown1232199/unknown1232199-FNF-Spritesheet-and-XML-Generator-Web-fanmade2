<script lang="ts">
    export let id: string;
    export let curChoice: string;
    export let text: string;
</script>

<button
    type="button"
    class:active={id === curChoice}
    aria-pressed={id === curChoice}
    on:click={() => { curChoice = id; }}
>
    {text}
</button>

<style>
    button {
        min-width: 0;
        min-height: 3rem;
        padding: .7rem 1rem;
        border: 1px solid var(--border-color);
        border-radius: 10px 10px 0 0;
        background: var(--background);
        color: var(--text-color);
        font: inherit;
        font-weight: 650;
        cursor: pointer;
        transition: background-color .16s ease, border-color .16s ease, transform .16s ease;
    }
    button:hover {
        background: var(--secondary-button);
        border-color: var(--secondary-button-emphasized);
    }
    button:focus-visible {
        outline: 3px solid var(--secondary-button-emphasized);
        outline-offset: 2px;
        position: relative;
        z-index: 1;
    }
    button.active {
        background: var(--secondary-button-emphasized);
        color: var(--text-color-2);
        border-bottom-color: var(--secondary-button-emphasized);
    }
</style>