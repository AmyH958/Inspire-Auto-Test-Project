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

WebUI.setText(findTestObject('Object Repository/流通/指定參考書/新增指定參考書/Page_(15trunk)/input_(15trunk)_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/流通/指定參考書/新增指定參考書/Page_(15trunk)/input_(15trunk)_member_pwd'), 'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/新增指定參考書/Page_(15trunk)/input_(15trunk)_Submit'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/指定參考書/新增指定參考書/Page_(15trunk)/a_'), '流通')

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/新增指定參考書/Page_(15trunk)/a_'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/指定參考書/新增指定參考書/Page_(15trunk)/a__1'), '指定參考書')

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/新增指定參考書/Page_(15trunk)/a__1'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/新增指定參考書/Page_(15trunk)/a__1_2'))

WebUI.switchToWindowTitle('iNspire')

WebUI.verifyElementText(findTestObject('Object Repository/流通/指定參考書/新增指定參考書/Page_iNspire/td_'), '*課程名稱')

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/新增指定參考書/Page_iNspire/img__Any_3'))

WebUI.selectOptionByValue(findTestObject('Object Repository/流通/指定參考書/新增指定參考書/Page_iNspire/select_()() (A) (A) (A) (A)PickListValues.1_9ff1e5'), 
    '0', true)

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/新增指定參考書/Page_iNspire/a_'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/新增指定參考書/Page_iNspire/input__TextField'))

WebUI.setText(findTestObject('Object Repository/流通/指定參考書/新增指定參考書/Page_iNspire/input__TextField'), 'A11234')

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/新增指定參考書/Page_iNspire/img__datePickerImg'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/新增指定參考書/Page_iNspire/button_5 , 2024'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/指定參考書/新增指定參考書/Page_iNspire/td__1'), '*單位所系')

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/新增指定參考書/Page_iNspire/img__Any_0_0'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/新增指定參考書/Page_iNspire/a__1'))

WebUI.selectOptionByValue(findTestObject('Object Repository/流通/指定參考書/新增指定參考書/Page_iNspire/select_12'), '1', true)

WebUI.setText(findTestObject('Object Repository/流通/指定參考書/新增指定參考書/Page_iNspire/input__AccessionNumberID'), '011010174707')

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/新增指定參考書/Page_iNspire/input__buttonSubmit'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/指定參考書/新增指定參考書/Page_iNspire/nobr_'), '教師證號')

WebUI.verifyElementText(findTestObject('Object Repository/流通/指定參考書/新增指定參考書/Page_iNspire/span_A11234'), 'A11234')

WebUI.verifyElementText(findTestObject('Object Repository/流通/指定參考書/新增指定參考書/Page_iNspire/nobr__1'), '條碼號')

WebUI.verifyElementText(findTestObject('Object Repository/流通/指定參考書/新增指定參考書/Page_iNspire/span_011010174707'), '011010174707')

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/新增指定參考書/Page_iNspire/input_MSTK -_savechecked'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/指定參考書/新增指定參考書/Page_iNspire/p_'), '存檔成功')

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/新增指定參考書/Page_iNspire/a__1_2'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/新增指定參考書/Page_iNspire/a__1_2_3'))

WebUI.switchToDefaultContent()

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/新增指定參考書/Page_(15trunk)/a__1_2_3'))

WebUI.closeBrowser()

