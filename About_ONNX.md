---
icon: tools
order: 2
---

### What is ONNX?
ONNX (Open Neural Network Exchange) is an open-source standard for representing deep learning models. It was developed to facilitate interoperability between various deep learning frameworks, such as TensorFlow, PyTorch, and Caffe2. ONNX provides a common file format that allows models trained in one framework to be used in another framework for inference, without the need for model conversion.

### Why do we use ONNX?
ONNX is used for several reasons:

**Interoperability**: ONNX enables developers to train models in one deep learning framework and use them in another for inference, without the need for conversion. This eases the process of deploying models and allows developers to use the best tools for each part of the development process.

**Portability**: ONNX provides a standardized format for deep learning models, making it easier to share models across different platforms and devices.

**Optimization**: ONNX-compatible tools and libraries, such as ONNX Runtime, can provide optimizations for specific hardware, leading to improved performance.

**Ecosystem support**: Many popular deep learning frameworks and tools, like TensorFlow, PyTorch, and Microsoft's ONNX Runtime, support ONNX, providing a broad range of options for developers.

### How to create an ONNX file
Check out our (pyezkl)[https://github.com/zkonduit/pyezkl] repository to find detailed steps on generating an ONNX file.



resources:
https://github.com/onnx/onnx
https://github.com/zkonduit/pyezkl