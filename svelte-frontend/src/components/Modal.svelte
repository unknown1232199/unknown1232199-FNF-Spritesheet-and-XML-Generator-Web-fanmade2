<script lang="ts">
    export let showModal = false;
    export let onClose = () => {};
    export let closeButtonMsg = "Close";

    function closeModal() {
        showModal = false;
        onClose();
    }

    function handleBackdrop(event: MouseEvent) {
        if (event.target === event.currentTarget) closeModal();
    }
</script>

{#if showModal}
    <div class="modal-backdrop" role="presentation" on:click={handleBackdrop} on:keydown={(event) => event.key === "Escape" && closeModal()}>
        <section class="modal-window" role="dialog" aria-modal="true" on:click|stopPropagation>
            <slot name="header" />
            <hr />
            <slot />
            <hr />
            <slot name="accept-btn" />
            <hr />
            <button type="button" on:click={closeModal}>{closeButtonMsg}</button>
        </section>
    </div>
{/if}

<style>
    .modal-backdrop {
        position: fixed;
        inset: 0;
        z-index: 10000;
        display: flex;
        align-items: center;
        justify-content: center;
        padding: 16px;
        background: rgba(0, 0, 0, .68);
        overflow-y: auto;
        pointer-events: auto;
    }
    .modal-window {
        box-sizing: border-box;
        width: min(720px, 100%);
        max-height: min(90vh, 900px);
        overflow-y: auto;
        padding: 18px;
        border: 1px solid var(--border-color);
        border-radius: 12px;
        background: var(--background);
        color: var(--text-color);
        box-shadow: 0 20px 70px #0009;
        pointer-events: auto;
    }
    .modal-window > :global(button) {
        margin: 3px;
    }
</style>
