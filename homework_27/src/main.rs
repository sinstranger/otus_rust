// + Возвращать итератор по всем элементам,
// + добавлять элемент в конец,
// + добавлять элемент в начало,
// + добавлять элемент после N-го,
// + Разделяться на два списка: от начального элемента до (N-1)-го и от (N-1)-го до последнего.
// + Предоставлять возможность изменять элементы списка.

// Так как каждый элемент списка содержит ссылку на следующий, Rust не даст нам менять элементы списка (правило заимствования о одной мутабельной ссылке).
// Для преодоления этого ограничения можно использовать обёртку Rc<RefCell>. Она даст возможность модифицировать элемент списка несмотря на то, что на
// него существует ссылка (у предыдущего элемента).

mod linked_list;
mod tests;

use linked_list::LinkedList;

fn main() {
    let mut linked_list: LinkedList<u32> = LinkedList::default();
    let _len = linked_list.len();
    linked_list.push_tail(1);
    linked_list.push_head(10);
    linked_list.push_head(100);
    linked_list.insert_by_index(22, 2);
    linked_list.update_by_index(23, 2);
    let second_part = linked_list.split_by_index(2).1;

    for i in second_part.iter() {
        println!("{:?}", i)
    }
}
