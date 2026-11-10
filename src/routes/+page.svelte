<script lang="ts">
    import '../styles.css';
    import { onMount } from "svelte";
    import { mediaControls } from "tauri-plugin-media-api";

    interface Track {
        title: string;
        artist?: string;
        album?: string;
        artworkUrl?: string;
        artworkData?: string;
    }

    let track = $state<Track | null>(null);
    let isPlaying = $state(false);

    async function updateMedia() {
        // Retrieve track metadata
        try {
            const metadata = await mediaControls.getMetadata();
            track = metadata ?? null;
        } catch (error) {
            console.error("Metadata error:", error);
        }

        // Retrieve playback status
        try {
            const status = await mediaControls.getPlaybackStatus();

            console.log("Playback status:", status);

            isPlaying = status === "playing";
        } catch (error) {
            console.error("Playback error:", error);
        }
    }

    onMount(() => {
        updateMedia();
        const interval = setInterval(updateMedia, 2000);
        return () => clearInterval(interval);
    });

</script>

<div class=centered>  
<div id="title">
{`${track?.title ?? "none"} - ${track?.artist ?? "unknown"}`}
</div> 
<div class="recordplayer" >
<img id="vinyl" class:playing={isPlaying} src="./vinyl.png" alt="" 
width = 300, height = 300>
<img id="vinyloverlay" src="./vinyloverlay.png" alt="" width = 300, 
    height = 300>
{#if track?.artworkData}
<img id="cover" class:playing={isPlaying} src= {track.artworkData.
            startsWith("data:")
                ? track.artworkData
                : `data:image/jpeg;base64,${track.artworkData}`
            } alt="Album cover" width = 100, height = 100 />
        {:else if track?.artworkUrl}
            <img id="cover" class:playing={isPlaying} src={track.artworkUrl} 
            alt="Album cover" width = 100, height = 100 />
    {:else}
        <p>none</p>
    {/if}
    </div>
</div>
