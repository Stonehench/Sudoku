import 'package:flutter/material.dart';

class DigitSelect extends StatelessWidget {
  final Object? size;
  const DigitSelect(this.size, {super.key, required this.selectDigit});

  final Function(int) selectDigit;

  @override
  Widget build(BuildContext context) {
    double fontSize = size! as int <= 9
        ? 30.0
        : size as int <= 16
            ? 15.0
            : 6.0;
    return SizedBox(
      height: 50,
      width: 340,
      child: Container(
        alignment: Alignment.center,
        //color: const Color.fromARGB(255, 178, 195, 233),
        child: ListView.builder(
          physics: const NeverScrollableScrollPhysics(),
          scrollDirection: Axis.horizontal,
          itemCount: size as int,
          padding: const EdgeInsets.all(2),
          itemBuilder: (BuildContext context, int index) {
            return InkWell(
              onTap: () => print(index + 1),
              child: Container(
                  alignment: Alignment.center,
                  height: 50,
                  width: 340 / (size as int),
                  child: Text((index + 1).toString(),
                      style: TextStyle(
                        fontSize: fontSize,
                        color: Colors.black,
                      ))),
            );
          },
        ),
      ),
    );
  }
}
