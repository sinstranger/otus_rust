// + Возвращать итератор по всем элементам,
// добавлять элемент в конец,
// + добавлять элемент в начало,
// добавлять элемент после N-го,
// Разделяться на два списка: от начального элемента до (N-1)-го и от (N-1)-го до последнего.
// Предоставлять возможность изменять элементы списка.

// Так как каждый элемент списка содержит ссылку на следующий, Rust не даст нам менять элементы списка (правило заимствования о одной мутабельной ссылке).
// Для преодоления этого ограничения можно использовать обёртку Rc<RefCell>. Она даст возможность модифицировать элемент списка несмотря на то, что на
//  него существует ссылка (у предыдущего элемента).

mod linked_list;
mod tests;

use linked_list::LinkedList;

fn main() {
    let mut empty_list: LinkedList<u32> = LinkedList::default();
    // println!("Empty list: {:?}", empty_list);

    empty_list.push_head(1);
    empty_list.push_head(2);
    empty_list.push_head(3);
    empty_list.push_head(4);
    // println!("Empty list: {:?}", empty_list);

    for i in empty_list.iter() {
        println!("{:?}", i)
    }
}
