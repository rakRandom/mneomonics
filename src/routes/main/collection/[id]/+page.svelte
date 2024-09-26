<script lang="ts">
    import type { Question } from '$src/ts/types';

    export let data;
    console.log(data);

    let currentQuestion: Question | undefined = undefined;

    function optionSelected() {
        if (currentQuestion) {
            currentQuestion = undefined;
            return;
        }
        currentQuestion = {
            header: "Question 1 - OBMEP - 2024",
            statement: [
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc fringilla mauris id ante sagittis convallis. Praesent pellentesque, justo varius ultricies convallis, felis arcu hendrerit nulla, vel lobortis lorem massa eget urna. Nam vehicula orci sit amet magna viverra, eget efficitur nulla tempus. Nullam molestie velit nec massa aliquam condimentum. Duis sit amet auctor nulla, nec euismod odio. Vestibulum in turpis non lorem eleifend vestibulum sed sed felis.",
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc fringilla mauris id ante sagittis convallis. Praesent pellentesque, justo varius ultricies convallis, felis arcu hendrerit nulla, vel lobortis lorem massa eget urna. Nam vehicula orci sit amet magna viverra, eget efficitur nulla tempus. Nullam molestie velit nec massa aliquam condimentum. Duis sit amet auctor nulla, nec euismod odio. Vestibulum in turpis non lorem eleifend vestibulum sed sed felis."
            ],
            imageURL: undefined,
            alternatives: [
                "Lorem ipsum",
                "Lorem ipsum",
                "Lorem ipsum",
                "Lorem ipsum",
            ],
            correctAlt: 0
        };
    }

    function learnButton() {
        // window.location.href = "main/collection/1";
    }
</script>

<div class="flex h-screen w-full overflow-hidden">
    <a 
        href=".."
        class="
            z-50 absolute top-4 left-6 text-center
            py-1 w-[75px] 
            colored-button"
        >
        Back
    </a>
    
    <button 
        type="button" 
        class="
            z-50 absolute top-0 right-6 
            py-1 w-[100px] h-fit 
            colored-button bg-[var(--color-1)]
            transition-transform translate-y-4 duration-1000"
        class:translate-y-10={currentQuestion}
        class:delay-700={currentQuestion}
        on:click={learnButton}>
        Learn
    </button>

    <div 
        class="absolute translate-x-32 translate-y-4 font-semibold text-2xl transition-transform duration-1000"
        class:translate-x-[calc(50vw-50%)]={!currentQuestion}
        class:delay-700={!currentQuestion}
    >
        Collection's Name
    </div>
    
    <div 
        class="
            w-[380px] h-[calc(100%-5rem)] 
            mt-16 
            regular-border rounded-2xl overflow-hidden 
            translate-x-6 transition-transform duration-1000"
        class:translate-x-[calc(50vw-50%)]={!currentQuestion}
        class:delay-700={!currentQuestion}
        >
        <div class="flex flex-col gap-2 h-full w-full px-6 py-4 overflow-y-auto">
            {#each Array(32).keys() as _}
            <button 
                type="button" 
                class="
                    text-left truncate 
                    px-3 py-1.5 w-full min-h-fit 
                    regular-border regular-button 
                    transition-transform hover:translate-x-1.5"
                on:click={optionSelected}
                >
                Question Number {_}
            </button>
            {/each}
        </div>
    </div>

    <div 
        class="
            ml-auto p-6 pt-4 pr-2 w-[calc(100vw-380px-3rem)] h-[calc(100%-2.5rem)] 
            regular-border border-r-0 rounded-3xl rounded-r-none 
            translate-y-6 transition-transform duration-1000"
        class:translate-y-[100vh]={!currentQuestion}
        class:delay-700={currentQuestion}
        >
        {#if currentQuestion}
        <div class="pr-2 w-[calc(100%)] h-full overflow-y-auto">
            <h1 class="font-semibold text-2xl indent-4 truncate w-[calc(100%-100px-3rem)]">
                {currentQuestion.header ?? ""}
            </h1>

            <hr class="regular-hr mt-4 mb-8">
            
            <ol class="flex flex-col gap-4 text-[#DDD]">
                {#each currentQuestion.statement as paragraph}
                <li>
                    <p class="indent-4 hover:text-white">
                        {paragraph}
                    </p>
                </li>
                {/each}
            </ol>

            {#if currentQuestion.imageURL}
            <img src={currentQuestion.imageURL} alt="If you can see this, you're fucked up.">
            {/if}

            <hr class="regular-hr my-8">

            <ol class="flex flex-col gap-2">
                {#each currentQuestion.alternatives as alternative}
                <li>
                    <button 
                        type="button"
                        class="
                            block 
                            px-2 py-1.5 mx-auto w-3/4 
                            regular-border regular-button 
                            transition-transform hover:-translate-y-0.5"
                        >
                        {alternative}
                    </button>
                </li>
                {/each}
            </ol>
            <p class="text-center mt-4 opacity-50 select-none">
                Right answer: {currentQuestion.correctAlt + 1}
            </p>
        </div>
        {/if}
    </div>
</div>

<style>
    
</style>
