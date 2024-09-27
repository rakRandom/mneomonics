<script lang="ts">
    import type { Question } from '$src/ts/types';

    export let data;
    console.log(data);

    let currentQuestion: Question | undefined = undefined;
    let questionBuffer: Question | undefined = undefined;

    function optionSelected() {
        if (currentQuestion) {
            currentQuestion = undefined;
            return;
        }
        currentQuestion = {
            header: "Question 1 - OBMEP - 2024",
            statement: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc fringilla mauris id ante sagittis convallis. Praesent pellentesque, justo varius ultricies convallis, felis arcu hendrerit nulla, vel lobortis lorem massa eget urna. Nam vehicula orci sit amet magna viverra, eget efficitur nulla tempus. Nullam molestie velit nec massa aliquam condimentum. Duis sit amet auctor nulla, nec euismod odio. Vestibulum in turpis non lorem eleifend vestibulum sed sed felis.",
            imageURL: undefined,
            alternatives: [
                "Lorem ipsum",
                "Lorem ipsum",
                "Lorem ipsum",
                "Lorem ipsum",
            ],
            correctAlt: 0
        };

        questionBuffer = currentQuestion;
    }

    function newFlashCard() {

    }

    function saveButton() {

    }

    function learnButton() {
        // window.location.href = "main/collection/1";
    }
</script>

<div class="flex h-screen w-full overflow-hidden">
    <!-- Back Button -->
    <a 
        href=".."
        class="
            z-50 absolute top-4 left-6 text-center
            py-1 w-[75px] 
            colored-button"
        >
        Back
    </a>
    
    <!-- Learn Button -->
    <button 
        type="button" 
        class="
            z-50 absolute top-0 right-6 
            mt-[2px] py-1 w-[100px] h-fit 
            colored-button bg-[var(--color-1)]
            transition-transform translate-y-4 duration-1000"
        class:translate-y-10={currentQuestion}
        class:delay-700={currentQuestion}
        on:click={learnButton}>
        Learn
    </button>

    <!-- Title / Name of the collection -->
    <div 
        class="absolute translate-x-32 translate-y-4 font-semibold text-2xl transition-transform duration-1000"
        class:translate-x-[calc(50vw-50%)]={!currentQuestion}
        class:delay-700={!currentQuestion}
    >
        Collection's Name
    </div>

    <!-- Informations about the collection -->
    <div 
        class="absolute top-16 left-6 w-[360px] h-[calc(100%-5rem)] transition-transform duration-1000"
        class:-translate-x-[calc(100%+6rem)]={currentQuestion}
        class:delay-1000={!currentQuestion}
        >
        <div class="w-full h-fit regular-border p-4">
            <h2 class="font-semibold text-2xl">
                Collection Status
            </h2>
            <hr class="regular-hr my-2">
            <div role="table" class="flex w-full flex-1 px-2 overflow-y-auto">
                <ol class="w-fit">
                    <li>Flash Cards:</li>
                    <li>Times seen:</li>
                    <li>Last seen (days):</li>
                </ol>

                <ol class="flex-1 text-right">
                    <li>4</li>
                    <li>0</li>
                    <li>7</li>
                </ol>
            </div>
        </div>
    </div>
    
    <!-- List of questions -->
    <div 
        class="
            flex flex-col
            w-[380px] h-[calc(100%-5rem)] 
            mt-16 
            regular-border rounded-2xl overflow-hidden 
            translate-x-6 transition-transform duration-1000"
        class:translate-x-[calc(50vw-50%)]={!currentQuestion}
        class:delay-700={!currentQuestion}
        >
        <div class="w-full px-6 pt-4">
            <button 
                on:click={newFlashCard}
                type="button" 
                class="
                    px-3 py-1.5 w-full min-h-fit 
                    regular-border colored-button 
                    transition-transform hover:-translate-y-0.5">
                New Flash Card
            </button>
        </div>
        <hr class="regular-hr mx-6 mt-4">
        <div class="flex flex-col gap-2 w-full px-6 py-4 overflow-y-auto">
            {#each Array(32).keys() as _}
            <button 
                type="button" 
                class="
                    flex justify-between text-left truncate 
                    px-3 py-1.5 w-full min-h-fit 
                    regular-border regular-button 
                    transition-transform hover:translate-x-1.5"
                on:click={optionSelected}
                >
                <span>Question Number {_}</span>
                <button class="delete-button px-1.5 bg-[var(--color-1)] hover:bg-[var(--color-3)] rounded-md"></button>
            </button>
            {/each}
        </div>
    </div>

    <!-- Question window -->
    <div 
        class="
            ml-auto p-6 pt-4 pr-2 w-[calc(100vw-380px-3rem)] h-[calc(100%-2.5rem)] 
            regular-border border-r-0 rounded-3xl rounded-r-none 
            translate-y-6 transition-transform duration-1000"
        class:translate-y-[100vh]={!currentQuestion}
        class:delay-700={currentQuestion}
        >
        {#if questionBuffer}
        <div class="pr-2 w-[calc(100%)] h-full overflow-y-auto">
            <div class="flex w-[calc(100%-120px)]">
                <input 
                    spellcheck="false"
                    autocomplete="off"
                    type="text" 
                    id="question-title-input"
                    class="flex-1 font-semibold text-2xl indent-4 truncate bg-transparent outline-none" 
                    bind:value={questionBuffer.header}>
                <button on:click={saveButton} type="button" class="px-3 py-1 regular-button regular-border">
                    Save
                </button>
            </div>

            <hr class="regular-hr mt-4 mb-8">
            
            <textarea 
                spellcheck="false"
                autocomplete="off"
                id="statement-input"
                class="indent-4 text-[#DDD] hover:text-white bg-transparent outline-none"
                bind:value={questionBuffer.statement}
            ></textarea>

            {#if questionBuffer.imageURL}
            <img src={questionBuffer.imageURL} alt="If you can see this, you're fucked up.">
            {/if}

            <hr class="regular-hr my-8">

            <ol class="flex flex-col gap-2">
                {#each questionBuffer.alternatives as alternative, index}
                <li>
                    <input 
                        spellcheck="false"
                        autocomplete="off"
                        type="text"
                        id="alternative-input-{index}"
                        class="
                            block text-center
                            px-2 py-1.5 mx-auto w-3/4 
                            regular-border regular-button 
                            bg-transparent outline-none
                            transition-transform hover:-translate-y-0.5"
                        bind:value={alternative}
                        >
                </li>
                {/each}
            </ol>
            <div class="text-center mt-4 opacity-50">
                Right answer: 
                <input 
                    spellcheck="false"
                    autocomplete="off"
                    type="text"
                    id="correct-alt-input"
                    class="bg-transparent outline-none"
                    bind:value={questionBuffer.correctAlt}>
            </div>
        </div>
        {/if}
    </div>
</div>

<style>
    .delete-button::after {
        content: '×';
    }

    #statement-input {
        resize: none;
    }

    #statement-input,
    #correct-alt-input {
        field-sizing: content;
    }
</style>
