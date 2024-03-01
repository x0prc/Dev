export default function ListPicker({ values }) {
   const randIdx = Math.floor(Math.random() * values.length);
   const randElement = values[randIdx];
return (
    <div>
        <p> list of values: {values} </p>
        <p> random element is: {randElement} </p>
    </div>
)
}
