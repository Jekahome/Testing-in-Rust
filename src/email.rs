#![allow(unused_imports)]

#[cfg(test)]
use mockall::{automock, mock, predicate::*};

pub mod email{
    use super::*;

    #[cfg_attr(test, automock)]
    pub trait SendMessage{
        fn self_send_message(&self,msg:&str)->bool;
    }

    pub struct Sender;
    impl SendMessage for Sender{
        fn self_send_message(&self,_msg:&str)->bool{
            // ... hard or slow work with side effect
            true
        }
    }

    pub struct SendMessageService<T:SendMessage>{
       pub sender:T
    }
    impl<T:SendMessage> SendMessageService<T>{
        pub fn self_send_message(&self,msg:&str)->bool{
            self.sender.self_send_message(msg)
        }
    }
 
    pub fn send_message_g<T:SendMessage>(msg:&str,service:&T)->bool{
        service.self_send_message(msg)
    }

    /*
    Привязывание к конкретному типу Sender лишает нас Mocking тестирования самого SendMessageService

    pub struct SendMessageService{
        sender:Sender
    }
    */
}

/*
    Mocking тестирование

    $ cargo nextest run email::
*/
#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use email::{SendMessageService,Sender,SendMessage,send_message_g,MockSendMessage};

    #[test]
    fn no_mocking_send_message_g_success() {
        let sender = Sender;
        assert!(send_message_g("msg",&sender));

        let service = SendMessageService{sender};
        assert!(service.self_send_message("msg"));
        
    }

    mod for_moking{
        use super::*;
        pub struct SenderMoking;
        impl SendMessage for SenderMoking{
            fn self_send_message(&self,_msg:&str)->bool{
                true
            }
        } 
    }

    #[test]
    fn mocking_send_message_success() {
        use for_moking::SenderMoking;

        let sender = SenderMoking;
        assert!(send_message_g("msg",&sender));
        
        let service = SendMessageService{sender};
        assert!(service.self_send_message("msg")); 
    }

    #[test]
    fn mocking_mockall_send_message_success() {
        let mut sender_mock = MockSendMessage::new();
        sender_mock.expect_self_send_message()
            .with(mockall::predicate::eq("msg"))
            .times(3)
            .returning(|_| true);
        assert!(send_message_g("msg",&sender_mock));

        assert!(sender_mock.self_send_message("msg"));

        let service = SendMessageService{sender:sender_mock};
        assert!(service.self_send_message("msg"));   
    }
}