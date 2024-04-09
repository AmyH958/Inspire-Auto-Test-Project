import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('http://10.11.20.15/inspireapp/logon')

WebUI.maximizeWindow()

WebUI.setText(findTestObject('Object Repository/統計/流通統計/讀者類型借閱暨歸還流通量統計報表/UI顯示正確/Page_(15trunk)/input_(15trunk)_member_id'), 
    'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/統計/流通統計/讀者類型借閱暨歸還流通量統計報表/UI顯示正確/Page_(15trunk)/input_(15trunk)_member_pwd'), 
    'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.enhancedClick(findTestObject('Object Repository/統計/流通統計/讀者類型借閱暨歸還流通量統計報表/UI顯示正確/Page_(15trunk)/input_(15trunk)_Submit'))

WebUI.verifyElementText(findTestObject('Object Repository/統計/流通統計/讀者類型借閱暨歸還流通量統計報表/UI顯示正確/Page_(15trunk)/a_'), '統計')

WebUI.enhancedClick(findTestObject('Object Repository/統計/流通統計/讀者類型借閱暨歸還流通量統計報表/UI顯示正確/Page_(15trunk)/a_'))

WebUI.verifyElementText(findTestObject('Object Repository/統計/流通統計/讀者類型借閱暨歸還流通量統計報表/UI顯示正確/Page_(15trunk)/a__1'), '流通統計')

WebUI.enhancedClick(findTestObject('Object Repository/統計/流通統計/讀者類型借閱暨歸還流通量統計報表/UI顯示正確/Page_(15trunk)/a__1'))

WebUI.verifyElementText(findTestObject('Object Repository/統計/流通統計/讀者類型借閱暨歸還流通量統計報表/UI顯示正確/Page_(15trunk)/a__1_2'), '讀者類型借閱暨歸還流通量統計報表')

WebUI.enhancedClick(findTestObject('Object Repository/統計/流通統計/讀者類型借閱暨歸還流通量統計報表/UI顯示正確/Page_(15trunk)/a__1_2'))

WebUI.setText(findTestObject('Object Repository/統計/流通統計/讀者類型借閱暨歸還流通量統計報表/UI顯示正確/Page_(15trunk)/input__startDate'), '2006/03/06')

WebUI.setText(findTestObject('Object Repository/統計/流通統計/讀者類型借閱暨歸還流通量統計報表/UI顯示正確/Page_(15trunk)/input__endDate'), '2024/03/27')

WebUI.enhancedClick(findTestObject('Object Repository/統計/流通統計/讀者類型借閱暨歸還流通量統計報表/UI顯示正確/Page_(15trunk)/input__selLocationId'))

WebUI.enhancedClick(findTestObject('Object Repository/統計/流通統計/讀者類型借閱暨歸還流通量統計報表/UI顯示正確/Page_(15trunk)/input__selLocationId'))

WebUI.enhancedClick(findTestObject('Object Repository/統計/流通統計/讀者類型借閱暨歸還流通量統計報表/UI顯示正確/Page_(15trunk)/input__selLocationId'))

WebUI.enhancedClick(findTestObject('Object Repository/統計/流通統計/讀者類型借閱暨歸還流通量統計報表/UI顯示正確/Page_(15trunk)/input__selLocationId'))

WebUI.enhancedClick(findTestObject('Object Repository/統計/流通統計/讀者類型借閱暨歸還流通量統計報表/UI顯示正確/Page_(15trunk)/img__open'))

WebUI.enhancedClick(findTestObject('Object Repository/統計/流通統計/讀者類型借閱暨歸還流通量統計報表/UI顯示正確/Page_(15trunk)/label'))

WebUI.enhancedClick(findTestObject('Object Repository/統計/流通統計/讀者類型借閱暨歸還流通量統計報表/UI顯示正確/Page_(15trunk)/button_'))

WebUI.enhancedClick(findTestObject('Object Repository/統計/流通統計/讀者類型借閱暨歸還流通量統計報表/UI顯示正確/Page_(15trunk)/input__elementName'))

WebUI.enhancedClick(findTestObject('Object Repository/統計/流通統計/讀者類型借閱暨歸還流通量統計報表/UI顯示正確/Page_(15trunk)/img__Any_0_0'))

WebUI.enhancedClick(findTestObject('Object Repository/統計/流通統計/讀者類型借閱暨歸還流通量統計報表/UI顯示正確/Page_(15trunk)/a__1_2_3'))

WebUI.enhancedClick(findTestObject('Object Repository/統計/流通統計/讀者類型借閱暨歸還流通量統計報表/UI顯示正確/Page_(15trunk)/input__SELALL'))

WebUI.enhancedClick(findTestObject('Object Repository/統計/流通統計/讀者類型借閱暨歸還流通量統計報表/UI顯示正確/Page_(15trunk)/input__formSubmitSearch'))

WebUI.enableSmartWait()

WebUI.enhancedClick(findTestObject('Object Repository/統計/流通統計/讀者類型借閱暨歸還流通量統計報表/UI顯示正確/Page_(15trunk)/a__1_2_3_4'))

WebUI.closeBrowser()

