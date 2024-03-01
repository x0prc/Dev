import { useState } from "react";
function changeNum (){

}
export default function Counter() {
    const [num, setNum] = useState(5);

    const changeNum = () => {
        setNum(num + 1);
    }
    return (
        <div>
            <p> The count is: {num} </p>
            <button onClick={changeNum}>Increment</button>
        </div>
    );
}
//props dont change, they are immutable.
//states can show change in the DOM.