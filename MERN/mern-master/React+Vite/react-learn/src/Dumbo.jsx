import { useState } from "react";

function generateGameBoard() {
    console.log('gamer moment')
   return Array(5000);
}

export default function Dumbo() {
    const [board, setBoard] = useState(generateGameBoard);
    return <button onClick={() => setBoard("hello")}>click to change state</button>
}

