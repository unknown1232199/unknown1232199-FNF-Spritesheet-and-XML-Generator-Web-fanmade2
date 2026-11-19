<script lang="ts">
    export let id: string;
    export let curChoice: string;
    export let text: string;
</script>
<button class:active-tab={id === curChoice} aria-pressed={id === curChoice} on:click={() => { curChoice = id; }}>
    <span class="tab-indicator"></span>{text}
</button>
<style>
    button { min-height:43px; display:flex; justify-content:center; align-items:center; gap:9px; border:1px solid transparent; border-radius:8px; background:transparent; color:var(--muted,#9ba4b8); font-size:12px; font-weight:700; transition:background .15s,border-color .15s,color .15s; }
    button:hover { background:#9c8cff12; color:var(--text-color); }
    button.active-tab { background:#9c8cff1b; border-color:#9c8cff55; color:var(--text-color); }
    .tab-indicator { width:6px; height:6px; border-radius:50%; background:transparent; }
    button.active-tab .tab-indicator { background:#9c8cff; box-shadow:0 0 8px #9c8cff; }
    @media(max-width:530px) { button { font-size:11px; padding:8px 5px; } }
</style>