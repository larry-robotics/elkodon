#[generic_tests::define]
mod reactor {
    use elkodon_bb_container::semantic_string::*;
    use elkodon_bb_posix::file_descriptor::FileDescriptorBased;
    use elkodon_bb_posix::unique_system_id::UniqueSystemId;
    use elkodon_bb_system_types::file_name::FileName;
    use elkodon_bb_testing::{assert_that, test_requires};
    use elkodon_cal::event::unix_datagram_socket::*;
    use elkodon_cal::event::{Listener, ListenerBuilder, Notifier, NotifierBuilder};
    use elkodon_cal::reactor::{Reactor, *};
    use elkodon_pal_posix::posix::POSIX_SUPPORT_AT_LEAST_TIMEOUTS;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::Barrier;
    use std::time::{Duration, Instant};

    const TIMEOUT: Duration = Duration::from_millis(50);
    const INFINITE_TIMEOUT: Duration = Duration::from_secs(3600 * 24);
    const NUMBER_OF_ATTACHMENTS: usize = 64;

    struct NotifierListenerPair {
        notifier: unix_datagram_socket::Notifier<u64>,
        listener: unix_datagram_socket::Listener<u64>,
    }

    impl NotifierListenerPair {
        fn new() -> Self {
            let name = generate_name();
            let listener = unix_datagram_socket::ListenerBuilder::<u64>::new(&name)
                .create()
                .unwrap();
            let notifier = unix_datagram_socket::NotifierBuilder::<u64>::new(&name)
                .open()
                .unwrap();

            Self { listener, notifier }
        }
    }

    fn generate_name() -> FileName {
        let mut file = FileName::new(b"reactor_tests_").unwrap();
        file.push_bytes(
            UniqueSystemId::new()
                .unwrap()
                .value()
                .to_string()
                .as_bytes(),
        )
        .unwrap();
        file
    }

    #[test]
    fn attach_and_detach_works<Sut: Reactor>() {
        let sut = <<Sut as Reactor>::Builder>::new().create().unwrap();

        let mut listeners = vec![];
        let mut guards = vec![];
        for _ in 0..NUMBER_OF_ATTACHMENTS {
            let name = generate_name();
            listeners.push(
                unix_datagram_socket::ListenerBuilder::<u64>::new(&name)
                    .create()
                    .unwrap(),
            );
        }

        assert_that!(sut.is_empty(), eq true);
        for i in 0..NUMBER_OF_ATTACHMENTS {
            assert_that!(sut.len(), eq i);
            guards.push(sut.attach(&listeners[i]));
            assert_that!(sut.is_empty(), eq false);
        }

        for i in 0..NUMBER_OF_ATTACHMENTS {
            assert_that!(sut.len(), eq NUMBER_OF_ATTACHMENTS - i);
            assert_that!(sut.is_empty(), eq false);
            guards.pop();
        }
        assert_that!(sut.is_empty(), eq true);
        assert_that!(sut.len(), eq 0);
    }

    #[test]
    fn try_wait_does_not_block_when_triggered_single<Sut: Reactor>() {
        let sut = <<Sut as Reactor>::Builder>::new().create().unwrap();

        let attachment = NotifierListenerPair::new();
        attachment.notifier.notify(123).unwrap();

        let _guard = sut.attach(&attachment.listener);

        let mut triggered_fds = vec![];
        assert_that!(
            sut.try_wait(|fd| triggered_fds.push(unsafe { fd.native_handle() })),
            is_ok
        );

        assert_that!(triggered_fds, len 1);
        assert_that!(triggered_fds[0], eq unsafe { attachment. listener.file_descriptor().native_handle() });
    }

    #[test]
    fn timed_wait_does_not_block_when_triggered_single<Sut: Reactor>() {
        let sut = <<Sut as Reactor>::Builder>::new().create().unwrap();

        let attachment = NotifierListenerPair::new();
        attachment.notifier.notify(123).unwrap();

        let _guard = sut.attach(&attachment.listener);

        let mut triggered_fds = vec![];
        assert_that!(
            sut.timed_wait(
                |fd| triggered_fds.push(unsafe { fd.native_handle() }),
                INFINITE_TIMEOUT
            ),
            is_ok
        );

        assert_that!(triggered_fds, len 1);
        assert_that!(triggered_fds[0], eq unsafe { attachment. listener.file_descriptor().native_handle() });
    }

    #[test]
    fn blocking_wait_does_not_block_when_triggered_single<Sut: Reactor>() {
        let sut = <<Sut as Reactor>::Builder>::new().create().unwrap();

        let attachment = NotifierListenerPair::new();
        attachment.notifier.notify(123).unwrap();

        let _guard = sut.attach(&attachment.listener);

        let mut triggered_fds = vec![];
        assert_that!(
            sut.blocking_wait(|fd| triggered_fds.push(unsafe { fd.native_handle() }),),
            is_ok
        );

        assert_that!(triggered_fds, len 1);
        assert_that!(triggered_fds[0], eq unsafe { attachment. listener.file_descriptor().native_handle() });
    }

    #[test]
    fn try_wait_activates_as_long_as_there_is_data_to_read<Sut: Reactor>() {
        let sut = <<Sut as Reactor>::Builder>::new().create().unwrap();

        let attachment = NotifierListenerPair::new();
        attachment.notifier.notify(123).unwrap();

        let _guard = sut.attach(&attachment.listener);

        for _ in 0..4 {
            let mut triggered_fds = vec![];
            assert_that!(
                sut.try_wait(|fd| triggered_fds.push(unsafe { fd.native_handle() })),
                is_ok
            );

            assert_that!(triggered_fds, len 1);
            assert_that!(triggered_fds[0], eq unsafe { attachment. listener.file_descriptor().native_handle() });
        }

        attachment.listener.try_wait().unwrap();
        let mut triggered_fds = vec![];
        assert_that!(
            sut.try_wait(|fd| triggered_fds.push(unsafe { fd.native_handle() })),
            is_ok
        );

        assert_that!(triggered_fds, is_empty);
    }

    #[test]
    fn timed_wait_activates_as_long_as_there_is_data_to_read<Sut: Reactor>() {
        let sut = <<Sut as Reactor>::Builder>::new().create().unwrap();

        let attachment = NotifierListenerPair::new();
        attachment.notifier.notify(123).unwrap();

        let _guard = sut.attach(&attachment.listener);

        for _ in 0..4 {
            let mut triggered_fds = vec![];
            assert_that!(
                sut.timed_wait(
                    |fd| triggered_fds.push(unsafe { fd.native_handle() }),
                    INFINITE_TIMEOUT
                ),
                is_ok
            );

            assert_that!(triggered_fds, len 1);
            assert_that!(triggered_fds[0], eq unsafe { attachment. listener.file_descriptor().native_handle() });
        }

        attachment.listener.try_wait().unwrap();
        let mut triggered_fds = vec![];
        assert_that!(
            sut.try_wait(|fd| triggered_fds.push(unsafe { fd.native_handle() })),
            is_ok
        );

        assert_that!(triggered_fds, is_empty);
    }

    #[test]
    fn blocking_wait_activates_as_long_as_there_is_data_to_read<Sut: Reactor>() {
        let sut = <<Sut as Reactor>::Builder>::new().create().unwrap();

        let attachment = NotifierListenerPair::new();
        attachment.notifier.notify(123).unwrap();

        let _guard = sut.attach(&attachment.listener);

        for _ in 0..4 {
            let mut triggered_fds = vec![];
            assert_that!(
                sut.blocking_wait(|fd| triggered_fds.push(unsafe { fd.native_handle() }),),
                is_ok
            );

            assert_that!(triggered_fds, len 1);
            assert_that!(triggered_fds[0], eq unsafe { attachment. listener.file_descriptor().native_handle() });
        }

        attachment.listener.try_wait().unwrap();
        let mut triggered_fds = vec![];
        assert_that!(
            sut.try_wait(|fd| triggered_fds.push(unsafe { fd.native_handle() })),
            is_ok
        );

        assert_that!(triggered_fds, is_empty);
    }

    #[test]
    fn try_wait_does_not_block_when_triggered_many<Sut: Reactor>() {
        let sut = <<Sut as Reactor>::Builder>::new().create().unwrap();

        let mut attachments = vec![];
        for _ in 0..NUMBER_OF_ATTACHMENTS {
            let attachment = NotifierListenerPair::new();
            attachment.notifier.notify(123).unwrap();
            attachments.push(attachment);
        }

        let mut guards = vec![];
        for i in 0..NUMBER_OF_ATTACHMENTS {
            guards.push(sut.attach(&attachments[i].listener).unwrap());
        }

        let mut triggered_fds = vec![];
        assert_that!(
            sut.try_wait(|fd| triggered_fds.push(unsafe { fd.native_handle() })),
            is_ok
        );

        assert_that!(triggered_fds, len NUMBER_OF_ATTACHMENTS);
        for i in 0..NUMBER_OF_ATTACHMENTS {
            assert_that!(triggered_fds, contains unsafe { attachments[i].listener.file_descriptor().native_handle() } );
        }
    }

    #[test]
    fn timed_wait_does_not_block_when_triggered_many<Sut: Reactor>() {
        let sut = <<Sut as Reactor>::Builder>::new().create().unwrap();

        let mut attachments = vec![];
        for _ in 0..NUMBER_OF_ATTACHMENTS {
            let attachment = NotifierListenerPair::new();
            attachment.notifier.notify(123).unwrap();
            attachments.push(attachment);
        }

        let mut guards = vec![];
        for i in 0..NUMBER_OF_ATTACHMENTS {
            guards.push(sut.attach(&attachments[i].listener).unwrap());
        }

        let mut triggered_fds = vec![];
        assert_that!(
            sut.timed_wait(
                |fd| triggered_fds.push(unsafe { fd.native_handle() }),
                INFINITE_TIMEOUT
            ),
            is_ok
        );

        assert_that!(triggered_fds, len NUMBER_OF_ATTACHMENTS);
        for i in 0..NUMBER_OF_ATTACHMENTS {
            assert_that!(triggered_fds, contains unsafe { attachments[i].listener.file_descriptor().native_handle() } );
        }
    }

    #[test]
    fn blocking_wait_does_not_block_when_triggered_many<Sut: Reactor>() {
        let sut = <<Sut as Reactor>::Builder>::new().create().unwrap();

        let mut attachments = vec![];
        for _ in 0..NUMBER_OF_ATTACHMENTS {
            let attachment = NotifierListenerPair::new();
            attachment.notifier.notify(123).unwrap();
            attachments.push(attachment);
        }

        let mut guards = vec![];
        for i in 0..NUMBER_OF_ATTACHMENTS {
            guards.push(sut.attach(&attachments[i].listener).unwrap());
        }

        let mut triggered_fds = vec![];
        assert_that!(
            sut.blocking_wait(|fd| triggered_fds.push(unsafe { fd.native_handle() })),
            is_ok
        );

        assert_that!(triggered_fds, len NUMBER_OF_ATTACHMENTS);
        for i in 0..NUMBER_OF_ATTACHMENTS {
            assert_that!(triggered_fds, contains unsafe { attachments[i].listener.file_descriptor().native_handle() } );
        }
    }

    #[test]
    fn timed_wait_blocks_for_at_least_timeout<Sut: Reactor>() {
        test_requires!(POSIX_SUPPORT_AT_LEAST_TIMEOUTS);
        let sut = <<Sut as Reactor>::Builder>::new().create().unwrap();

        let attachment = NotifierListenerPair::new();

        let _guard = sut.attach(&attachment.listener);

        let mut triggered_fds = vec![];
        let start = Instant::now();
        assert_that!(
            sut.timed_wait(
                |fd| triggered_fds.push(unsafe { fd.native_handle() }),
                TIMEOUT
            ),
            is_ok
        );
        assert_that!(start.elapsed(), ge TIMEOUT);

        assert_that!(triggered_fds, len 0);
    }

    #[test]
    fn try_wait_triggers_until_all_data_is_consumed<Sut: Reactor>() {
        let sut = <<Sut as Reactor>::Builder>::new().create().unwrap();

        let mut attachments = vec![];
        for _ in 0..NUMBER_OF_ATTACHMENTS {
            let attachment = NotifierListenerPair::new();
            attachment.notifier.notify(123).unwrap();
            attachments.push(attachment);
        }

        let mut guards = vec![];
        for i in 0..NUMBER_OF_ATTACHMENTS {
            guards.push(sut.attach(&attachments[i].listener).unwrap());
        }

        for n in 0..NUMBER_OF_ATTACHMENTS {
            let mut triggered_fds = vec![];
            assert_that!(
                sut.try_wait(|fd| triggered_fds.push(unsafe { fd.native_handle() })),
                is_ok
            );

            assert_that!(triggered_fds, len NUMBER_OF_ATTACHMENTS - n);
            for i in n..NUMBER_OF_ATTACHMENTS {
                assert_that!(triggered_fds, contains unsafe { attachments[i].listener.file_descriptor().native_handle() } );
            }

            attachments[n].listener.try_wait().unwrap();
        }

        let mut triggered_fds = vec![];
        assert_that!(
            sut.try_wait(|fd| triggered_fds.push(unsafe { fd.native_handle() })),
            is_ok
        );

        assert_that!(triggered_fds, len 0);
    }

    #[test]
    fn timed_wait_triggers_until_all_data_is_consumed<Sut: Reactor>() {
        let sut = <<Sut as Reactor>::Builder>::new().create().unwrap();

        let mut attachments = vec![];
        for _ in 0..NUMBER_OF_ATTACHMENTS {
            let attachment = NotifierListenerPair::new();
            attachment.notifier.notify(123).unwrap();
            attachments.push(attachment);
        }

        let mut guards = vec![];
        for i in 0..NUMBER_OF_ATTACHMENTS {
            guards.push(sut.attach(&attachments[i].listener).unwrap());
        }

        for n in 0..NUMBER_OF_ATTACHMENTS {
            let mut triggered_fds = vec![];
            assert_that!(
                sut.timed_wait(
                    |fd| triggered_fds.push(unsafe { fd.native_handle() }),
                    INFINITE_TIMEOUT
                ),
                is_ok
            );

            assert_that!(triggered_fds, len NUMBER_OF_ATTACHMENTS - n);
            for i in n..NUMBER_OF_ATTACHMENTS {
                assert_that!(triggered_fds, contains unsafe { attachments[i].listener.file_descriptor().native_handle() } );
            }

            attachments[n].listener.try_wait().unwrap();
        }

        let mut triggered_fds = vec![];
        assert_that!(
            sut.try_wait(|fd| triggered_fds.push(unsafe { fd.native_handle() })),
            is_ok
        );

        assert_that!(triggered_fds, len 0);
    }

    #[test]
    fn blocking_wait_triggers_until_all_data_is_consumed<Sut: Reactor>() {
        let sut = <<Sut as Reactor>::Builder>::new().create().unwrap();

        let mut attachments = vec![];
        for _ in 0..NUMBER_OF_ATTACHMENTS {
            let attachment = NotifierListenerPair::new();
            attachment.notifier.notify(123).unwrap();
            attachments.push(attachment);
        }

        let mut guards = vec![];
        for i in 0..NUMBER_OF_ATTACHMENTS {
            guards.push(sut.attach(&attachments[i].listener).unwrap());
        }

        for n in 0..NUMBER_OF_ATTACHMENTS {
            let mut triggered_fds = vec![];
            assert_that!(
                sut.blocking_wait(|fd| triggered_fds.push(unsafe { fd.native_handle() })),
                is_ok
            );

            assert_that!(triggered_fds, len NUMBER_OF_ATTACHMENTS - n);
            for i in n..NUMBER_OF_ATTACHMENTS {
                assert_that!(triggered_fds, contains unsafe { attachments[i].listener.file_descriptor().native_handle() } );
            }

            attachments[n].listener.try_wait().unwrap();
        }

        let mut triggered_fds = vec![];
        assert_that!(
            sut.try_wait(|fd| triggered_fds.push(unsafe { fd.native_handle() })),
            is_ok
        );

        assert_that!(triggered_fds, len 0);
    }

    #[test]
    fn timed_wait_blocks_until_triggered<Sut: Reactor>() {
        let name = generate_name();
        let barrier = Barrier::new(2);
        let counter = AtomicU64::new(0);

        std::thread::scope(|s| {
            let t = s.spawn(|| {
                let sut = <<Sut as Reactor>::Builder>::new().create().unwrap();
                let listener = unix_datagram_socket::ListenerBuilder::<u64>::new(&name)
                    .create()
                    .unwrap();
                let _guard = sut.attach(&listener);
                barrier.wait();

                let mut triggered_fds = vec![];
                let timed_wait_result = sut.timed_wait(
                    |fd| triggered_fds.push(unsafe { fd.native_handle() }),
                    INFINITE_TIMEOUT,
                );

                counter.fetch_add(1, Ordering::Relaxed);

                assert_that!(triggered_fds, len 1);
                assert_that!(timed_wait_result, is_ok);
            });

            barrier.wait();
            std::thread::sleep(TIMEOUT);
            assert_that!(counter.load(Ordering::Relaxed), eq 0);

            let notifier = unix_datagram_socket::NotifierBuilder::<u64>::new(&name)
                .open()
                .unwrap();
            notifier.notify(123).unwrap();
            t.join().unwrap();
        });
    }

    #[test]
    fn blocking_wait_blocks_until_triggered<Sut: Reactor>() {
        let name = generate_name();
        let barrier = Barrier::new(2);
        let counter = AtomicU64::new(0);

        std::thread::scope(|s| {
            let t = s.spawn(|| {
                let sut = <<Sut as Reactor>::Builder>::new().create().unwrap();
                let listener = unix_datagram_socket::ListenerBuilder::<u64>::new(&name)
                    .create()
                    .unwrap();
                let _guard = sut.attach(&listener);
                barrier.wait();

                let mut triggered_fds = vec![];
                let blocking_wait_result =
                    sut.blocking_wait(|fd| triggered_fds.push(unsafe { fd.native_handle() }));

                counter.fetch_add(1, Ordering::Relaxed);

                assert_that!(triggered_fds, len 1);
                assert_that!(blocking_wait_result, is_ok);
            });

            barrier.wait();
            std::thread::sleep(TIMEOUT);
            assert_that!(counter.load(Ordering::Relaxed), eq 0);

            let notifier = unix_datagram_socket::NotifierBuilder::<u64>::new(&name)
                .open()
                .unwrap();
            notifier.notify(123).unwrap();
            t.join().unwrap();
        });
    }

    #[instantiate_tests(<elkodon_cal::reactor::posix_select::Reactor>)]
    mod posix_select {}
}
