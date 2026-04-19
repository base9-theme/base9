import Color from 'color';
import { destritured } from 'unused-package';

// comment
/* inline */
/*
 * multi
 * line
 */

/**
 * JsDoc
 * @param c1 explain
 * @param c2 explain.
 * @returns return value
 */
function function_name(c1: Color, c2: Color): string {
    return c1.hex();
}

const constant: number[] = [0, 1, 2, 3, 4, 5];
let variable = "string";
class ClassName extends Color {
    public field2: number = 1;
    constructor(public argument1: string) {
        super();
    }
}