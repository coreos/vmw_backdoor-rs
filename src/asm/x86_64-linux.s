.text

# fn _vmw_backdoor_lb_in(arg: &BackdoorBuf, res: &mut BackdoorBuf);
.global _vmw_backdoor_lb_in
_vmw_backdoor_lb_in:
  # Preserve caller values.
  movq %rbx, %r8;
  movq %rbp, %r9;
  movq %rdi, %r10;
  movq %rsi, %r11;

  movl 0(%r10), %eax;
  movl 4(%r10), %ebx;
  movl 8(%r10), %ecx;
  movl 12(%r10), %edx;
  movl 16(%r10), %ebp;
  movl 20(%r10), %edi;
  movl 24(%r10), %esi;

  # Backdoor call.
  inl  %dx, %eax;

  # Record results.
  movl %eax, 0(%r11);
  movl %ebx, 4(%r11);
  movl %ecx, 8(%r11);
  movl %edx, 12(%r11);
  movl %ebp, 16(%r11);
  movl %edi, 20(%r11);
  movl %esi, 24(%r11);

  # Restore caller values.
  movq %r8, %rbx;
  movq %r9, %rbp;

  # Return.
  xor %rax, %rax;
  ret;

# fn _vmw_backdoor_lb_out(arg: &BackdoorBuf, res: &mut BackdoorBuf);
.global _vmw_backdoor_lb_out
_vmw_backdoor_lb_out:
  # Preserve caller values.
  movq %rbx, %r8;
  movq %rbp, %r9;
  movq %rdi, %r10;
  movq %rsi, %r11;

  # Magic number.
  movl 0(%r10), %eax;
  movl 4(%r10), %ebx;
  # Command.
  movl 8(%r10), %ecx;
  # I/O port (low-bandwidth).
  movl 12(%r10), %edx;
  movl 16(%r10), %ebp;
  movl 20(%r10), %edi;
  movl 24(%r10), %esi;

	# Backdoor call.
  outl  %eax, %dx;

  # Record results.
  movl %eax, 0(%r11);
  movl %ebx, 4(%r11);
  movl %ecx, 8(%r11);
  movl %edx, 12(%r11);
  movl %ebp, 16(%r11);
  movl %edi, 20(%r11);
  movl %esi, 24(%r11);

  # Restore caller values.
  movq %r8, %rbx;
  movq %r9, %rbp;

  # Return.
  xor %rax, %rax;
  ret;

# fn _vmw_backdoor_hb_out(arg: &Buf, cmd: &mut Buf);
.global _vmw_backdoor_hb_out
_vmw_backdoor_hb_out:
	  # Preserve caller values.
	  movq %rbx, %r8;
	  movq %rbp, %r9;
	  movq %rdi, %r10;
	  movq %rsi, %r11;

	  movl 0(%r10), %eax;
	  movl 4(%r10), %ebx;
	  movl 8(%r10), %ecx;
	  movl 12(%r10), %edx;
	  movl 16(%r10), %ebp;
	  movq 20(%r10), %rdi;
	  movq 28(%r10), %rsi;

    # Backdoor call.
    cld;
    rep;
    outsb;

	  # Record results.
	  movl %eax, 0(%r11);
	  movl %ebx, 4(%r11);
	  movl %ecx, 8(%r11);
	  movl %edx, 12(%r11);
	  movl %ebp, 16(%r11);
	  movq %rdi, 20(%r11);
	  movq %rsi, 28(%r11);

	  # Restore caller values.
	  movq %r8, %rbx;
	  movq %r9, %rbp;

	  # Return.
  	xor %rax, %rax;
	  ret;

# fn _vmw_backdoor_hb_in(arg: &Buf, cmd: &mut Buf);
.global _vmw_backdoor_hb_in
_vmw_backdoor_hb_in:
	  # Preserve caller values.
	  movq %rbx, %r8;
	  movq %rbp, %r9;
	  movq %rdi, %r10;
	  movq %rsi, %r11;

	  movl 0(%r10), %eax;
	  movl 4(%r10), %ebx;
	  movl 8(%r10), %ecx;
	  movl 12(%r10), %edx;
	  movl 16(%r10), %ebp;
	  movq 20(%r10), %rdi;
	  movq 28(%r10), %rsi;

    # Backdoor call.
    cld;
	  rep;
	  insb;

	  # Record results.
	  movl %eax, 0(%r11);
	  movl %ebx, 4(%r11);
	  movl %ecx, 8(%r11);
	  movl %edx, 12(%r11);
	  movl %ebp, 16(%r11);
	  movq %rdi, 20(%r11);
	  movq %rsi, 28(%r11);

	  # Restore caller values.
	  movq %r8, %rbx;
	  movq %r9, %rbp;

 	  # Return.
  	xor %rax, %rax;
	  ret;
