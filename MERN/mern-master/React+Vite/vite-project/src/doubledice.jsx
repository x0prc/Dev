export default function DoubleDice() {
    const num1 = Math.floor(Math.random() * 3) + 1;
    const num2 = Math.floor(Math.random() * 3) + 1;
    const isWinner = num1 === num2;
    const styles = { color: num1 === num2 ? "green" : "red" };
    return (
        <div>
            <h2>{num1 === num2 ? "you win" : "you lose"}</h2>
            <p>Num1: {num1} </p>
            <p>Num2: {num2} </p>
        </div>
    );
}
