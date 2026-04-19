import Color from 'color';
import _ from 'lodash';
export const tmp: any = {};
(window as any).tmp = tmp;
tmp.gen1 = (c: Color, r: number) => {
    const lab = c.lab().array();
    lab[0] *= 0.8;
    const lab3 = lab.slice();
    lab3[0] = 100-lab[0];
    const rtn = _.times(7, i => {
        const th = i/7*Math.PI*2;
        const lab2 = lab.slice();
        lab2[1] += Math.sin(th)*r;
        lab2[2] += Math.cos(th)*r;
        return Color.lab(lab2).hex().substring(1);
    }).join("-");
    console.log("https://coolors.co/"+Color.lab(lab3).hex().substring(1)+"-"+c.hex().substring(1)+"-"+rtn);
};
