import './App.css';
import ColorBox from './colorbox';

const data = [
  {id: 1, item: "eggs", quantity: 12, completed: false},
  {id: 2, item: "milk", quantity: 1, completed: true},
  {id: 3, item: "carrots", quantity: 13, completed: true},
];



function App() {
  return (
    <div>  
      <ColorBox/>
     </div>
  );
}
export default App;
