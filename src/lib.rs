#![allow(unused_variables)]

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    // 构造结构体
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    // 添加po文
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // 返回po文
    pub fn content(&self) -> &str {
         self.state.as_ref().unwrap().content(self)
    }

    // 状态1到状态2
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    // 状态2到状态3
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

// 状态机接口
trait State {
    // 申请审核
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    // 通过审核
    fn approve(self: Box<Self>) -> Box<dyn State>;
    // 调用state中对应值的 content 方法
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}



// 状态1 - 用结构体当状态
struct Draft {}

impl State for Draft {
    // 切换至状态2
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    // 切换为自身
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

// 状态2
struct PendingReview {}

impl State for PendingReview {
    // 切换为自身
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // 切换至状态3
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

// 状态3
struct Published {}

impl State for Published {
    // 切换为自身
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // 切换为自身
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    // 只有最后一个状态才需要特别的实现，其余使用默认
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
