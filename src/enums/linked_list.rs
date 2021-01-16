// use List::*;

// https://hanbum.gitbooks.io/rustbyexample/content/custom_types/enum/testcase_linked_list.html

pub enum List {
    // 요소 및 다음 노드 포인터를 튜플로 보관한다.
    Cons(u32, Box<List>),
    Nil, // linked list 의 끝
}

impl List {
    pub fn new() -> List {
        List::Nil
    }

    #[allow(clippy::clippy::needless_return)]
    pub fn prepend(self, elem: u32) -> List {
        return List::Cons(elem, Box::new(self));
    }

    #[allow(clippy::needless_return)]
    pub fn len(&self) -> u32 {
        return match *self {
            // 꼬리에 대한 소유권은 'self' 가 대여(borrow?) 중 이므로 얻을 수 없다.
            // 대신에 참조를 빌리도록 한다.
            List::Cons(_, ref tail) => 1 + tail.len(),
            // 기본 상태: 빈 list 는 0 의 길이를 갖는다.
            List::Nil => 0,
        };
    }

    #[allow(clippy::needless_return)]
    #[allow(clippy::useless_format)]
    pub fn stringify(&self) -> String {
        return match *self {
            List::Cons(head, ref tail) => format!("{}, {}", head, tail.stringify()),
            List::Nil => format!("Nil"),
        };
    }
}
