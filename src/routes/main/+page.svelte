<script lang="ts">
    type Question = {
        header: string,
        statement: string[],
        imageURL: string | undefined,
        alternatives: string[],
        correctAlt: number
    };

    export let data;
    console.log(data);

    let currentQuestion: Question | undefined = {
        header: "Question 1 - OBMEP - 2024",
        statement: [
            "Lorem ipsum, dor sit amet."
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
</script>

<div class="flex h-screen w-full *:h-full">
    <div class="flex flex-col w-1/3">
        <div class="flex items-center px-6 py-4 shadow-lg">
            <button type="button" class="font-semibold py-1 w-[60px] h-fit regular-border regular-button">
                Exit
            </button>
            <h1 class="font-semibold text-3xl ml-4 pl-4 border-l-2 border-[var(--color-2)]">
                mNeomonics
            </h1>
            <button type="button" class="relative ml-auto h-[32px] aspect-square rounded-full">
                <img src="plus.png" alt=""> <!-- This version is ugly, change it later -->
                <div class="absolute top-0 w-full h-full rounded-full bg-black opacity-0 hover:opacity-25"></div>
            </button>
        </div>
        <hr class="regular-hr">
        <ul class="
                p-6 overflow-y-auto
                *:*:regular-button *:*:regular-border 
                *:*:block *:*:text-left *:*:w-full *:*:h-full
                *:*:mb-4 *:*:px-4 *:*:py-2
                *:*:truncate">
            {#each Array(30).keys() as register}
            <li>
                <button type="button">
                    Question {register + 1} - OBMEP - 2024
                </button>
            </li>
            {/each}
        </ul>
    </div>
    <div class="w-2/3 border-l-2 border-[var(--color-2)] rounded-l-lg shadow-lg">
        <div class="flex items-center gap-4 px-4 py-4">
            <h1 class="flex-1 text-2xl break-all">
                {currentQuestion?.header ?? "Question"}
            </h1>
            <button 
                type="button" 
                class="font-semibold py-1 w-[100px] h-fit regular-border hover:border-transparent hover:bg-[var(--color-3)]">
                Learn
            </button>
        </div>
        <hr class="regular-hr">
        <div class="flex flex-col gap-4 p-6">
            {#if currentQuestion !== undefined}
                {#each currentQuestion?.statement as paragraph}
                <p class="break-all">
                    {paragraph}
                </p>
                {/each}

                {#if currentQuestion?.imageURL}
                <img src={currentQuestion?.imageURL} alt="">
                {/if}

                <ul>
                    {#each currentQuestion?.alternatives as alternative, i}
                    <li>
                    {#if currentQuestion?.correctAlt == i}
                    [X]
                    {:else}
                    [&nbsp;&nbsp;]
                    {/if}
                    - {alternative}</li>
                    {/each}
                </ul>
            {/if}
        </div>
    </div>
</div>

<style>
    
</style>
