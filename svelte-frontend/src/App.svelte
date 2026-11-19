<script lang="ts">
    import AppTab from "./components/AppTab.svelte";
    import IcongridGenerator from "./tabs/IcongridGenerator.svelte";
    import SpritesheetGenerator from "./tabs/SpritesheetGenerator.svelte";
    import type { Wasm_T } from './global';
    export let wasm: Wasm_T;
    let characterName = '';
    let curChoice = "spsh";
    let isAdvanced = false;
    let isDark = true;
</script>
<svelte:head>
    <title>FNF Studio — Spritesheet & XML Generator</title>
    <meta name="description" content="Create and edit Friday Night Funkin' spritesheets, XML atlases and icongrids directly in your browser." />
    <meta name="theme-color" content="#11131b" />
</svelte:head>
<main class:advanced-mode={isAdvanced} class:basic-mode={!isAdvanced} class:light-mode={!isDark}>
    <header class="topbar">
        <a class="brand" href="#top" aria-label="FNF Studio home"><span class="brand-mark">F</span><span class="brand-copy"><strong>FNF STUDIO</strong><small>SPRITESHEET WORKSPACE</small></span></a>
        <div class="top-actions"><span class="mode-label">{isAdvanced ? 'ADVANCED' : 'BASIC'} MODE</span><button class="mode-toggle" type="button" on:click={() => isAdvanced = !isAdvanced} aria-pressed={isAdvanced}>{isAdvanced ? 'Basic mode' : 'Advanced mode'}</button><button class="theme-toggle" type="button" on:click={() => isDark = !isDark} aria-label="Toggle color theme">{isDark ? '☼' : '☾'}</button></div>
    </header>
    <section class="intro" id="top"><div><p class="eyebrow">MODDING TOOLKIT <span>•</span> BROWSER-BASED</p><h1>Spritesheet <em>&</em> XML Generator</h1><p class="intro-copy">Prepare character atlases, organize animation frames and build icongrids — all in one workspace.</p></div><div class="intro-note"><span class="status-dot"></span> Processing locally in your browser</div></section>
    <section class="project-bar" aria-label="Project settings"><div class="project-label"><span class="project-icon">▦</span><div><strong>Current project</strong><small>Set a name for your exported files</small></div></div><label class="name-field" for="char-name"><span>CHARACTER NAME</span><input type="text" name="char-name" id="char-name" bind:value={characterName} placeholder="e.g. Boyfriend" autocomplete="off" /></label></section>
    <nav class="workspace-tabs" aria-label="Generator"><AppTab id="spsh" bind:curChoice={curChoice} text="Spritesheet Generator" /><AppTab id="icongrid" bind:curChoice={curChoice} text="Icongrid Generator" /></nav>
    <section class="workspace">
        {#if curChoice == "spsh"}<div class="workspace-heading"><div><span class="section-kicker">01 / ATLAS WORKSPACE</span><h2>Spritesheet editor</h2></div><span class="workspace-tag">PNG + XML</span></div><SpritesheetGenerator charname={characterName} wasm={wasm} />
        {:else if curChoice == "icongrid"}<div class="workspace-heading"><div><span class="section-kicker">02 / ICON WORKSPACE</span><h2>Icongrid builder</h2></div><span class="workspace-tag">PNG</span></div><IcongridGenerator bind:charname={characterName} wasm={wasm} />{/if}
    </section>
    <footer><span>FNF STUDIO</span><span>Files stay on your device · No upload required</span></footer>
</main>
<style>
    main { --panel:#191c27; --panel-raised:#202432; --line:#303647; --muted:#9ba4b8; --accent:#9c8cff; --accent-2:#6ed9c2; min-height:100vh; max-width:1500px; margin:0 auto; padding:0 30px 28px; color:#edf0f8; }
    .topbar{height:76px;display:flex;align-items:center;justify-content:space-between;border-bottom:1px solid var(--line)}.brand{display:flex;align-items:center;gap:11px;color:inherit;text-decoration:none}.brand-mark{width:37px;height:37px;display:grid;place-items:center;border-radius:11px;background:linear-gradient(140deg,#a493ff,#6f62d8);color:white;font-weight:900;font-size:21px}.brand-copy{display:grid;gap:3px;letter-spacing:1.4px;font-size:12px}.brand-copy small{color:var(--muted);font-size:9px;letter-spacing:1.8px}.top-actions{display:flex;align-items:center;gap:10px}.mode-label{color:var(--muted);font-size:10px;letter-spacing:1.2px}
    button{cursor:pointer;border:1px solid var(--line);border-radius:9px;padding:9px 12px;background:var(--panel-raised);color:inherit;font-weight:600}button:hover{border-color:var(--accent)}.theme-toggle{width:37px;height:37px;padding:0;font-size:18px}
    .intro{display:flex;justify-content:space-between;align-items:flex-end;gap:20px;padding:38px 0 28px}.eyebrow,.section-kicker{color:var(--accent-2);font-size:10px;font-weight:800;letter-spacing:1.8px}.eyebrow span{color:var(--accent);padding:0 5px}h1{margin:9px 0 7px;font-size:clamp(27px,4vw,42px);line-height:1.1;letter-spacing:-1.5px}h1 em{color:var(--accent);font-style:normal}.intro-copy{margin:0;max-width:620px;color:var(--muted);font-size:14px;line-height:1.6}.intro-note{white-space:nowrap;color:var(--muted);font-size:11px;padding:10px 13px;border:1px solid var(--line);border-radius:30px}.status-dot{display:inline-block;width:7px;height:7px;margin-right:7px;border-radius:50%;background:var(--accent-2);box-shadow:0 0 10px #6ed9c277}
    .project-bar{display:flex;align-items:center;justify-content:space-between;gap:20px;padding:17px 19px;border:1px solid var(--line);border-radius:13px;background:var(--panel)}.project-label{display:flex;align-items:center;gap:12px}.project-icon{display:grid;place-items:center;width:36px;height:36px;border-radius:9px;background:#9c8cff1b;color:var(--accent);font-size:20px}.project-label div{display:grid;gap:4px}.project-label strong{font-size:13px}.project-label small{color:var(--muted);font-size:11px}.name-field{display:grid;gap:6px;width:min(360px,55%);color:var(--muted);font-size:9px;font-weight:800;letter-spacing:1.3px}.name-field input{width:100%;box-sizing:border-box;padding:10px 12px;border:1px solid var(--line);border-radius:8px;outline:none;background:var(--panel-raised);color:inherit;font:inherit;font-size:13px;letter-spacing:0}.name-field input:focus{border-color:var(--accent)}
    .workspace-tabs{display:grid;grid-template-columns:1fr 1fr;gap:8px;margin:24px 0 0;padding:6px;border:1px solid var(--line);border-radius:12px 12px 0 0;background:#141721}.workspace{padding:20px;border:1px solid var(--line);border-top:0;border-radius:0 0 13px 13px;background:var(--panel);min-height:400px}.workspace-heading{display:flex;justify-content:space-between;align-items:center;margin-bottom:17px}.workspace-heading h2{margin:5px 0 0;font-size:19px;letter-spacing:-.4px}.workspace-tag{padding:6px 9px;border:1px solid #6ed9c244;border-radius:20px;color:var(--accent-2);font-size:9px;font-weight:800;letter-spacing:1px}
    footer{display:flex;justify-content:space-between;padding:20px 3px 0;color:var(--muted);font-size:10px}footer span:first-child{letter-spacing:1.5px;font-weight:800}
    :global(body){margin:0;background:#11131b;font-family:Inter,ui-sans-serif,system-ui,-apple-system,BlinkMacSystemFont,"Segoe UI",sans-serif}:global(*){box-sizing:border-box}
    .light-mode{--panel:#fff;--panel-raised:#f1f2f8;--line:#dfe1eb;--muted:#687086;color:#242638;background:#f3f4fa}.light-mode .workspace-tabs{background:#e9eaf2}.light-mode .workspace{background:var(--panel)}.advanced-mode .workspace{box-shadow:inset 0 3px 0 #9c8cff}
    @media(max-width:700px){main{padding:0 13px 20px}.topbar{height:64px}.mode-label{display:none}.intro{display:block;padding:29px 0 20px}.intro-note{display:inline-block;margin-top:16px}.project-bar{align-items:stretch;flex-direction:column;gap:14px}.name-field{width:100%}.workspace{padding:12px}.workspace-heading h2{font-size:17px}footer{gap:10px;flex-wrap:wrap}}
</style>