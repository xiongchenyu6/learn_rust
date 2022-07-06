;;; Directory Local Variables
;;; For more information see (info "(emacs) Directory Variables")

((nil . ((dap-debug-template-configurations . (("rust debug"
        :type "lldb"
        :request "launch"
        :name "cpptools::Run Configuration"
        :gdbpath "rust-gdb"
        :lldbpath "rust-lldb"
        :target "${workspaceFolder}/target/debug/learn_rust"
        :cwd "${workspaceFolder}"))))))
