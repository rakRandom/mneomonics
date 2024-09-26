type Question = {
    header: string,
    statement: string[],
    imageURL: string | undefined,
    alternatives: string[],
    correctAlt: number
};

export type { Question };
