import 'package:flutter/material.dart';

/// 对话里的一条消息。
class ChatMessage {
  const ChatMessage({required this.text, this.fromUser = false, this.draft = false});

  final String text;
  final bool fromUser;

  /// 是不是「流程草案卡」——AI 出的流程，等人确认。
  final bool draft;
}

/// 对话：用来说目标、改流程。（见 doc/views/chat.md）
class ChatPanel extends StatelessWidget {
  const ChatPanel({
    super.key,
    required this.messages,
    required this.placeholder,
    this.enabled = false,
    this.onSend,
  });

  final List<ChatMessage> messages;
  final String placeholder;

  /// 命令行还没有对话这一路，先摆出来但不可用。
  final bool enabled;
  final ValueChanged<String>? onSend;

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Expanded(
          child: ListView(
            padding: const EdgeInsets.all(16),
            children: [
              for (final message in messages)
                Align(
                  alignment: message.fromUser
                      ? Alignment.centerRight
                      : Alignment.centerLeft,
                  child: _bubble(context, message),
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
                  enabled: enabled,
                  decoration: InputDecoration(
                    hintText: placeholder,
                    border: const OutlineInputBorder(),
                    isDense: true,
                  ),
                  onSubmitted: (value) {
                    if (enabled) onSend?.call(value);
                  },
                ),
              ),
              const SizedBox(width: 8),
              FilledButton(
                onPressed: enabled ? () {} : null,
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
