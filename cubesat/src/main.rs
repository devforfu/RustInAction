#[derive(Debug)]
struct CubeSat {
    id: u64,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

#[derive(Debug)]
struct Message {
    to: u64,
    content: String,
}

struct GroundStation;

impl Mailbox {
    fn new() -> Self { Self { messages: vec![] } }

    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    fn deliver(&mut self, recepient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recepient.id {
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }
        None
    }

    fn print_content(&self) {
        if self.messages.is_empty() {
            println!("Mailbox is empty.");
        } else {
            println!("Pending messages:");
            self.messages.iter().for_each(|msg| println!("to={}, content={:#?}", msg.to, msg.content));
        }
    }
}

impl GroundStation {
    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat { id: sat_id }
    }

    fn send(&self, mailbox: &mut Mailbox, msg: Message) {
        mailbox.post(msg);
    }
}

impl CubeSat {
    fn recv(&self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

fn fetch_ids() -> Vec<u64> { vec![1, 2, 3] }

fn main() {
    let base = GroundStation;

    let mut mailbox = Mailbox::new();

    for id in fetch_ids() {
        base.send(&mut mailbox, Message { to: id, content: String::from("hi!") });
    }

    base.send(&mut mailbox, Message { to: 1, content: String::from("extra") });

    for id in fetch_ids() {
        let sat = base.connect(id);
        if let Some(msg) = sat.recv(&mut mailbox) {
            println!("Satellite {:?} received a message: {:?}", sat, msg);
        }
    }

    mailbox.print_content();
}