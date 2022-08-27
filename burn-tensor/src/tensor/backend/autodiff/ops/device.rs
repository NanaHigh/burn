use crate::backend::Backend;
use crate::{execute_ops, register_ops};
use crate::{
    graph::ops::{UnaryOps, UnaryOpsNodeState},
    tensor::backend::autodiff::ADTensor,
};
use crate::{ops::TensorOpsDevice, Element};

register_ops!(
    ops UnaryOps,
    name ADTensorDeviceOps state B::Device,
    partial |
        device: &B::Device,
        state: &UnaryOpsNodeState<B::TensorPrimitive<D>, B::TensorPrimitive<D>>
    | {
        state.output.grad().to_device(device.clone())
    },
);

macro_rules! define_impl {
    (
        $backend:ty,
        $backend_inner:ty,
        $element:ident
    ) => {
        impl<E: $element, const D: usize> TensorOpsDevice<$backend, D>
            for <$backend as Backend>::TensorPrimitive<D>
        where
            E: Element,
        {
            fn device(&self) -> <$backend as Backend>::Device {
                TensorOpsDevice::device(&self.tensor())
            }

            fn to_device(
                &self,
                device: <$backend as Backend>::Device,
            ) -> ADTensor<D, $backend_inner> {
                let tensor = self.tensor();
                execute_ops!(
                    input self.node.clone(),
                    out TensorOpsDevice::to_device(&tensor, device),
                    ops ADTensorDeviceOps::<$backend_inner, D>::new(tensor.device()),
                )
            }
        }
    };
}

crate::register_tch!();
crate::register_ndarray!();