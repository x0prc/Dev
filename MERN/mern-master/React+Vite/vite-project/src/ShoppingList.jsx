function ShoppingList({ items }) {
return (
    <ul> 
        {items.map((i) => (
           <ShoppingListItem
            item= {i.item}
            quantity={i.quantity}
            completed={i.completed}
            />
        ))}
    </ul>
);
}
export default ShoppingList;