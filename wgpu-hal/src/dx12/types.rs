#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use windows::Win32::Graphics::Dxgi;

windows_core::imp::define_interface!(
    ISwapChainPanelNative,
    ISwapChainPanelNative_Vtbl,
    0x63aad0b8_7c24_40ff_85a8_640d944cc325
);
impl core::ops::Deref for ISwapChainPanelNative {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISwapChainPanelNative, windows_core::IUnknown);
impl ISwapChainPanelNative {
    pub unsafe fn SetSwapChain<P0>(&self, swap_chain: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Dxgi::IDXGISwapChain1>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetSwapChain)(
                windows_core::Interface::as_raw(self),
                swap_chain.param().abi(),
            )
        }
        .ok()
    }
}

#[repr(C)]
pub struct ISwapChainPanelNative_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetSwapChain: unsafe extern "system" fn(
        swap_chain_panel_native: *mut core::ffi::c_void,
        swap_chain: *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,

winapi::STRUCT! {
    struct D3D12_FEATURE_DATA_D3D12_OPTIONS3 {
        CopyQueueTimestampQueriesSupported: winapi::shared::minwindef::BOOL,
        CastingFullyTypedFormatSupported: winapi::shared::minwindef::BOOL,
        WriteBufferImmediateSupportFlags: D3D12_COMMAND_LIST_SUPPORT_FLAGS,
        ViewInstancingTier: D3D12_VIEW_INSTANCING_TIER,
        BarycentricsSupported: winapi::shared::minwindef::BOOL,
    }
}

winapi::ENUM! {
    enum D3D12_WAVE_MMA_TIER  {
        D3D12_WAVE_MMA_TIER_NOT_SUPPORTED = 0,
        D3D12_WAVE_MMA_TIER_1_0 = 10,
    }
}

winapi::ENUM! {
    enum D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER {
        D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER_0 = 0,
        // D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER_1,
        // D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER_2,
    }
}

winapi::STRUCT! {
    struct D3D12_FEATURE_DATA_D3D12_OPTIONS4 {
        MSAA64KBAlignedTextureSupported: winapi::shared::minwindef::BOOL,
        SharedResourceCompatibilityTier: D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER,
        Native16BitShaderOpsSupported: winapi::shared::minwindef::BOOL,
    }
}

winapi::STRUCT! {
    struct D3D12_FEATURE_DATA_D3D12_OPTIONS9 {
        MeshShaderPipelineStatsSupported: winapi::shared::minwindef::BOOL,
        MeshShaderSupportsFullRangeRenderTargetArrayIndex: winapi::shared::minwindef::BOOL,
        AtomicInt64OnTypedResourceSupported: winapi::shared::minwindef::BOOL,
        AtomicInt64OnGroupSharedSupported: winapi::shared::minwindef::BOOL,
        DerivativesInMeshAndAmplificationShadersSupported: winapi::shared::minwindef::BOOL,
        WaveMMATier: D3D12_WAVE_MMA_TIER,
    }
}

winapi::ENUM! {
    enum D3D_SHADER_MODEL {
        D3D_SHADER_MODEL_NONE = 0,
        D3D_SHADER_MODEL_5_1 = 0x51,
        D3D_SHADER_MODEL_6_0 = 0x60,
        D3D_SHADER_MODEL_6_1 = 0x61,
        D3D_SHADER_MODEL_6_2 = 0x62,
        D3D_SHADER_MODEL_6_3 = 0x63,
        D3D_SHADER_MODEL_6_4 = 0x64,
        D3D_SHADER_MODEL_6_5 = 0x65,
        D3D_SHADER_MODEL_6_6 = 0x66,
        D3D_SHADER_MODEL_6_7 = 0x67,
        D3D_HIGHEST_SHADER_MODEL = 0x67,
    }
}

winapi::STRUCT! {
    struct D3D12_FEATURE_DATA_SHADER_MODEL {
        HighestShaderModel: D3D_SHADER_MODEL,
    }
}
