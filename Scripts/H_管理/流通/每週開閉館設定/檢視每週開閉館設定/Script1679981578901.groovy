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

WebUI.click(findTestObject('Object Repository/管理/流通/每週開閉館設定/input_(15trunk)_member_id'))

WebUI.setText(findTestObject('Object Repository/管理/流通/每週開閉館設定/input_(15trunk)_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/管理/流通/每週開閉館設定/input_(15trunk)_member_pwd'), 'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.click(findTestObject('Object Repository/管理/流通/每週開閉館設定/input_(15trunk)_Submit'))

WebUI.click(findTestObject('Object Repository/管理/流通/每週開閉館設定/a_'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/每週開閉館設定/a_'), '管理')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/每週開閉館設定/a__1'), '流通')

WebUI.scrollToElement(findTestObject('管理/流通/每週開閉館設定/a__1'), 2)

WebUI.navigateToUrl('http://10.11.20.15/inspireapp/weekly_schedule?init=1')

WebUI.enableSmartWait()

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/每週開閉館設定/div_'), '管理 > 流通 > 每周開閉館設定')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/每週開閉館設定/a__1_2_3'), '全部打開')

WebUI.click(findTestObject('Object Repository/管理/流通/每週開閉館設定/a__1_2_3'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/每週開閉館設定/div_LOCATIONS'), 'LOCATIONS')

WebUI.takeFullPageScreenshotAsCheckpoint('全部打開')

WebUI.verifyElementPresent(findTestObject('Object Repository/管理/流通/每週開閉館設定/img_LOCATIONS_jd1'), 0)

WebUI.verifyElementPresent(findTestObject('Object Repository/管理/流通/每週開閉館設定/img_LOCATIONS_jd1'), 0)

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/每週開閉館設定/a__1_2_3_4'), '全部關閉')

WebUI.click(findTestObject('Object Repository/管理/流通/每週開閉館設定/a__1_2_3_4'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/每週開閉館設定/div_LOCATIONS'), 'LOCATIONS')

WebUI.verifyElementPresent(findTestObject('Object Repository/管理/流通/每週開閉館設定/img_LOCATIONS_jd1_1'), 0)

WebUI.takeFullPageScreenshotAsCheckpoint('全部關閉')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/每週開閉館設定/a__1_2_3_4_5'), '神資圖書館')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/每週開閉館設定/a__1_2_3_4_5_6'), '星期一')

WebUI.click(findTestObject('Object Repository/管理/流通/每週開閉館設定/a__1_2_3_4_5_6'))

WebUI.verifyElementClickable(findTestObject('Object Repository/管理/流通/每週開閉館設定/a__1_2_3_4_5_6'))

WebUI.click(findTestObject('Object Repository/管理/流通/每週開閉館設定/a__1_2_3_4_5_6'))

WebUI.click(findTestObject('Object Repository/管理/流通/每週開閉館設定/td_'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/每週開閉館設定/td_'), '開放時間:')

WebUI.click(findTestObject('Object Repository/管理/流通/每週開閉館設定/input__DropdownTimePicker'))

WebUI.rightClick(findTestObject('Object Repository/管理/流通/每週開閉館設定/input__DropdownTimePicker'))

WebUI.click(findTestObject('Object Repository/管理/流通/每週開閉館設定/div_126172839410511003005351040154520502555_eaab9c'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/每週開閉館設定/td__1'), '隔夜借閱開始時段:')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/每週開閉館設定/td_'), '開放時間:')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/每週開閉館設定/td__1_2'), '閉館時間:')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/每週開閉館設定/td__1_2_3'), '隔夜借閱到期時段:')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/每週開閉館設定/td__1_2_3_4'), '已結束:')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/每週開閉館設定/td__1_2_3_4_5'), '註記:')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/每週開閉館設定/a__1_2_3_4_5_6_7'), '修改/存檔')

WebUI.click(findTestObject('Object Repository/管理/流通/每週開閉館設定/a__1_2_3_4_5_6_7'))

WebUI.rightClick(findTestObject('Object Repository/管理/流通/每週開閉館設定/a__1_2_3_4_5_6_7_8'))

WebUI.waitForAlert(2)

WebUI.verifyElementText(findTestObject('管理/流通/每週開閉館設定/td__1_2_3_4_5_6'), '存檔成功')

WebUI.click(findTestObject('Object Repository/管理/流通/每週開閉館設定/a__1_2_3_4_5_6_7_8'))

WebUI.click(findTestObject('Object Repository/管理/流通/每週開閉館設定/a__1_2_3_4_5_6_7_8_9'))

WebUI.click(findTestObject('Object Repository/管理/流通/每週開閉館設定/a__1_2_3_4_5_6_7_8_9_10'))

WebUI.click(findTestObject('Object Repository/管理/流通/每週開閉館設定/a__1_2_3_4_5_6_7_8_9_10_11'))

WebUI.click(findTestObject('Object Repository/管理/流通/每週開閉館設定/a__1_2_3_4_5_6_7_8_9_10_11_12'))

WebUI.click(findTestObject('Object Repository/管理/流通/每週開閉館設定/a__1_2_3_4_5_6_7_8_9_10_11_12_13'))

WebUI.click(findTestObject('Object Repository/管理/流通/每週開閉館設定/a__1_2_3_4_5_6_7_8_9_10_11_12_13_14'))

WebUI.click(findTestObject('Object Repository/管理/流通/每週開閉館設定/a__1_2_3_4_5_6_7_8_9_10_11_12_13_14_15'))

WebUI.verifyElementClickable(findTestObject('Object Repository/管理/流通/每週開閉館設定/a__1_2_3_4_5_6_7_8_9_10_11_12_13_14_15'))

WebUI.click(findTestObject('Object Repository/管理/流通/每週開閉館設定/a__1_2_3_4_5_6_7_8_9_10_11_12_13_14_15'))

WebUI.rightClick(findTestObject('Object Repository/管理/流通/每週開閉館設定/input__DropdownTimePicker'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/每週開閉館設定/input__DropdownTimePicker'), '')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/每週開閉館設定/input__DropdownTimePicker_1'), '')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/每週開閉館設定/input__DropdownTimePicker_0'), '')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/每週開閉館設定/input__DropdownTimePicker_2'), '')

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/每週開閉館設定/textarea__TextArea'), '')

WebUI.verifyElementPresent(findTestObject('Object Repository/管理/流通/每週開閉館設定/td_126172839410511003005351040154520502555a_233479'), 
    0)

WebUI.click(findTestObject('Object Repository/管理/流通/每週開閉館設定/a__1_2_3_4_5_6_7_8_9_10_11_12_13_14_15_16'))

WebUI.closeBrowser()

