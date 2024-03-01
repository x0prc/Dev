export default function Die() {
    const roll = Math.floor(Math.random() * 6) + 1;
    return <p>Die Roll: {roll}</p>
}