import 'package:flutter/material.dart';

import '../../infrastructure/host/host.dart';

/// 对话里的一条消息。
class ChatMessage {
  const ChatMessage({
    required this.text,
    this.fromUser = false,
    this.draft = false,
  });

  final String text;
  final bool fromUser;

  /// 是不是「流程草案卡」——AI 出的流程，等人确认。
  final bool draft;
}

/// 对话：用来说目标、改流程。（见 doc/views/chat.md）
///
/// 人跟本机的 pi 智能体说；话不落盘——程序不往产物里写一个字，
/// 要留痕是智能体按任务书自己写的事。
class ChatPanel extends StatefulWidget {
  const ChatPanel({
    super.key,
    required this.placeholder,
    required this.workingDir,
    this.opening = const [],
    this.brief = '',
    this.ask = runPiAsync,
  });

  final String placeholder;

  /// 起 pi 时的当前目录（工作区根）。
  final String workingDir;

  /// 开场那几句。
  final List<ChatMessage> opening;

  /// 每次发话时随带的背景：这一屏在看什么。
  final String brief;

  /// 起 pi 的那只手。测试里换成假的。
  final Future<({bool ran, String out})> Function(String prompt, String cwd)
  ask;

  @override
  State<ChatPanel> createState() => _ChatPanelState();
}

class _ChatPanelState extends State<ChatPanel> {
  late final List<ChatMessage> _messages = [...widget.opening];
  final TextEditingController _controller = TextEditingController();
  bool _asking = false;

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  Future<void> _send() async {
    final text = _controller.text.trim();
    if (text.isEmpty || _asking) return;
    setState(() {
      _messages.add(ChatMessage(text: text, fromUser: true));
      _asking = true;
    });
    _controller.clear();
    final prompt = widget.brief.isEmpty
        ? text
        : '${widget.brief}\n\n---\n\n$text';
    ({bool ran, String out}) reply;
    try {
      reply = await widget.ask(prompt, widget.workingDir);
    } catch (error) {
      reply = (ran: false, out: '$error');
    }
    if (!mounted) return;
    setState(() {
      _asking = false;
      _messages.add(
        ChatMessage(text: reply.ran ? reply.out : '没跑成：${reply.out}'),
      );
    });
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Expanded(
          child: ListView(
            padding: const EdgeInsets.all(16),
            children: [
              for (final message in _messages)
                Align(
                  alignment: message.fromUser
                      ? Alignment.centerRight
                      : Alignment.centerLeft,
                  child: _bubble(context, message),
                ),
              if (_asking)
                Align(
                  alignment: Alignment.centerLeft,
                  child: _bubble(context, const ChatMessage(text: '…在想')),
                ),
            ],
          ),
        ),
        Padding(
          padding: const EdgeInsets.all(12),
          child: Row(
            children: [
              Expanded(
                child: TextField(
                  controller: _controller,
                  enabled: !_asking,
                  decoration: InputDecoration(
                    hintText: widget.placeholder,
                    border: const OutlineInputBorder(),
                    isDense: true,
                  ),
                  onSubmitted: (_) => _send(),
                ),
              ),
              const SizedBox(width: 8),
              FilledButton(
                onPressed: _asking ? null : _send,
                child: const Text('发送'),
              ),
            ],
          ),
        ),
      ],
    );
  }

  Widget _bubble(BuildContext context, ChatMessage message) {
    return Container(
      margin: const EdgeInsets.only(bottom: 10),
      padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
      constraints: const BoxConstraints(maxWidth: 520),
      decoration: BoxDecoration(
        color: message.fromUser
            ? Theme.of(context).colorScheme.primaryContainer
            : Theme.of(context).colorScheme.surfaceContainerHighest,
        borderRadius: BorderRadius.circular(10),
      ),
      child: Text(message.text, style: Theme.of(context).textTheme.bodyMedium),
    );
  }
}
