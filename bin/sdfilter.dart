// Filter SDF blocks by score and write correct blocks to new SDF file
// Usage filterSDF -i 'Path to input dir' -o 'Output file+dir' -[lt/gt/eq] -f <Int>

import 'dart:io';
import 'package:args/args.dart';

List getFiles(String input) {
  // Check if input is filename
  if (input.contains('.sdf')) {
    var inputFile = File(input);
    var output = [inputFile];
    return output;
  } else {
    // input is Directory
    var inputDir = Directory(input);
    var inputArray = inputDir.listSync(recursive: false, followLinks: false);
    var output = [];
    // Filter out non sdf and hidden files in directory
    for (var item in inputArray) {
      if (item.toString().contains('.sdf') && !item.toString().contains('/.')) {
        output.add(item);
      }
    }
    return output;
  }
}

bool checkBlock(String SDFblock, String pattern, String operand, double limit) {
  if (SDFblock.split(pattern).length < 2) {
    print('Error parsing block:');
    print(SDFblock);
    print('Skipping');
    return false;
  } else {
    var val = double.tryParse(SDFblock.split(pattern)[1].split('>')[0].trim());
    if (val == null) {
      print('Error parsing block:');
      print(SDFblock);
      print('Skipping');
      return false;
    } else {
      if (operand == 'lt') {
        if (val < limit) {
          return true;
        } else {
          return false;
        }
      } else if (operand == 'eq') {
        if (val == limit) {
          return false;
        } else {
          return false;
        }
      } else if (operand == 'gt') {
        if (val > limit) {
          return true;
        } else {
          return false;
        }
      } else {
        print('Operator not specified or invalid');
        return false;
      }
    }
  }
}

void writeToOutput(String blockToWrite, String fileName) {
  var outputFile = File(fileName);
  if (outputFile.existsSync()) {
    outputFile.writeAsStringSync(blockToWrite, mode: FileMode.append);
  } else {
    outputFile.writeAsStringSync(blockToWrite.trimLeft(),
        mode: FileMode.append);
  }
}

void printHelp() {
  print('''
SDF file filter v0.1 beta
Usage: sdfilter -i /path/to/input(.sdf) -o /path/to/output.sdf -[l/e/g] -f <double>
Options:
    -i    --input               Input can be file or directory.
                                Absolute path must be specified.

    -o    --output              Output must be sdf file
                                Absolute path must be specified.
    
    -l    --lt                  Less than operand.
    -e    --eq                  Equal operand.
    -g    --gt                  Greater than operand.

    -f    --filter              Value to be compared to.

    -p    --pattern             SDF field to check. Ex: <SCORE>
                                Must end with > character
                                Defaults to <SCORE>''');
  exit(0);
}

void main(List<String> args) {
  var parser = ArgParser();
  parser.addOption('input', abbr: 'i');
  parser.addOption('output', abbr: 'o');
  parser.addOption('filter', abbr: 'f');
  parser.addOption('pattern', abbr: 'p', defaultsTo: '<SCORE>');
  parser.addFlag('lt', abbr: 'l');
  parser.addFlag('gt', abbr: 'g');
  parser.addFlag('eq', abbr: 'e');
  parser.addFlag('help', abbr: 'h');
  var arguments = parser.parse(args);

  // ignore: omit_local_variable_types
  double filter = 0.0;
  if (arguments['filter'] != null) {
    filter = double.tryParse(arguments['filter']);
  } else {
    printHelp();
  }
  if (!(arguments['lt'] || arguments['eq'] || arguments['gt'])) {
    printHelp();
  }

  var input = arguments['input'];
  var output = arguments['output'];
  var pattern = arguments['pattern'];

  // Help switch handler
  if (arguments['help'] ||
      arguments['input'] == null ||
      arguments['output'] == null) {
    printHelp();
  }

  var separator = '''
\$\$\$\$''';

  for (var fileName in getFiles(input)) {
    try {
      var contents;
      try {
        contents = fileName.readAsStringSync();
      } catch (e) {
        var fileNameString = fileName.toString().split(' ')[1].split("'")[1];
        contents = Process.runSync(
                'iconv', ['-f', 'UTF-8', '-t', 'UTF-8', '-c', fileNameString])
            .stdout;
      }
      var contentArray = contents.split('\$\$\$\$');
      contentArray.removeLast();
      for (var block in contentArray) {
        if (arguments['lt']) {
          if (checkBlock(block, pattern, 'lt', filter)) {
            writeToOutput(block + separator, output);
          }
        } else if (arguments['eq']) {
          if (checkBlock(block, pattern, 'eq', filter)) {
            writeToOutput(block + separator, output);
          }
        } else if (arguments['gt']) {
          if (checkBlock(block, pattern, 'gt', filter)) {
            writeToOutput(block + separator, output);
          }
        }
      }
    } catch (e) {
      print('Error opening file: ' + fileName.toString());
      print(e);
      print('Skipping!');
    }
  }
  print('');
  print('DONE!');
}
